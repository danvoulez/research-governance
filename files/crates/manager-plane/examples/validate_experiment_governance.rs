use manager_plane::BenchmarkRunManifest;
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandConfig {
    Preflight {
        charter: PathBuf,
        methodology: PathBuf,
        manifest: Option<PathBuf>,
        out: PathBuf,
    },
    Canonical {
        charter: PathBuf,
        methodology: PathBuf,
        out: PathBuf,
        checked_by: String,
        document_id: Option<String>,
    },
}

fn usage() -> &'static str {
    "usage:
  cargo run -p manager-plane --example validate_experiment_governance -- preflight \
    --charter /path/to/EXPERIMENT_CHARTER.md \
    --methodology /path/to/METHODOLOGY_SPEC.md \
    [--manifest /path/to/manifest.json] \
    --out /path/to/PRE_CYCLE_CHECKLIST.auto.md

  cargo run -p manager-plane --example validate_experiment_governance -- canonical \
    --charter /path/to/EXPERIMENT_CHARTER.md \
    --methodology /path/to/METHODOLOGY_SPEC.md \
    --out /path/to/CANONICAL_COMPLETENESS_CHECK.auto.md \
    [--checked-by codex] [--document-id cycle-2-governed-docs-auto]"
}

fn parse_cli() -> Result<Option<CommandConfig>, String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage().to_owned());
    };
    if matches!(command.as_str(), "--help" | "-h") {
        return Ok(None);
    }

    let mut charter = None;
    let mut methodology = None;
    let mut manifest = None;
    let mut out = None;
    let mut checked_by = Some("codex".to_owned());
    let mut document_id = None;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--charter" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--charter requires a path".to_owned())?;
                charter = Some(PathBuf::from(value));
            }
            "--methodology" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--methodology requires a path".to_owned())?;
                methodology = Some(PathBuf::from(value));
            }
            "--manifest" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--manifest requires a path".to_owned())?;
                manifest = Some(PathBuf::from(value));
            }
            "--out" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--out requires a path".to_owned())?;
                out = Some(PathBuf::from(value));
            }
            "--checked-by" => {
                checked_by = Some(
                    args.next()
                        .ok_or_else(|| "--checked-by requires a value".to_owned())?,
                );
            }
            "--document-id" => {
                document_id = Some(
                    args.next()
                        .ok_or_else(|| "--document-id requires a value".to_owned())?,
                );
            }
            "--help" | "-h" => return Ok(None),
            other => return Err(format!("unknown flag `{other}`\n\n{}", usage())),
        }
    }

    let charter = charter.ok_or_else(|| format!("missing --charter\n\n{}", usage()))?;
    let methodology = methodology.ok_or_else(|| format!("missing --methodology\n\n{}", usage()))?;
    let out = out.ok_or_else(|| format!("missing --out\n\n{}", usage()))?;

    match command.as_str() {
        "preflight" => Ok(Some(CommandConfig::Preflight {
            charter,
            methodology,
            manifest,
            out,
        })),
        "canonical" => Ok(Some(CommandConfig::Canonical {
            charter,
            methodology,
            out,
            checked_by: checked_by.unwrap_or_else(|| "codex".to_owned()),
            document_id,
        })),
        other => Err(format!(
            "unknown command `{other}`; expected `preflight` or `canonical`\n\n{}",
            usage()
        )),
    }
}

#[derive(Debug, Clone)]
struct DocBundle {
    charter: String,
    methodology: String,
}

fn load_docs(charter: &Path, methodology: &Path) -> Result<DocBundle, Box<dyn Error>> {
    Ok(DocBundle {
        charter: fs::read_to_string(charter)?,
        methodology: fs::read_to_string(methodology)?,
    })
}

fn normalized(text: &str) -> String {
    text.to_ascii_lowercase()
}

fn contains_any(text: &str, options: &[&str]) -> bool {
    options.iter().any(|option| text.contains(option))
}

fn section_body<'a>(text: &'a str, heading: &str) -> Option<&'a str> {
    let needle = format!("## {heading}");
    let start = text.find(&needle)?;
    let rest = &text[start + needle.len()..];
    let next = rest.find("\n## ").unwrap_or(rest.len());
    Some(rest[..next].trim())
}

fn first_meaningful_line(section: &str) -> Option<&str> {
    section
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('-') && !line.starts_with('>'))
        .or_else(|| {
            section
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty() && !line.starts_with('-'))
        })
}

fn line_like(section: &str) -> bool {
    let line = first_meaningful_line(section).unwrap_or("");
    !line.is_empty() && !line.contains('\n') && line.len() <= 220
}

fn has_field(text: &str, field: &str) -> bool {
    text.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with(&format!("- {field}:")) && trimmed != format!("- {field}:")
    })
}

fn bullet_count(section: &str) -> usize {
    section
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("- "))
        .count()
}

fn manifest_valid(path: &Path) -> bool {
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str::<BenchmarkRunManifest>(&raw).ok())
        .is_some()
}

fn yes_mark(value: bool) -> &'static str {
    if value { "[x]" } else { "[ ]" }
}

fn date_utc() -> String {
    let now = OffsetDateTime::now_utc().date();
    format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    )
}

fn write_output(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn render_preflight_markdown(
    docs: &DocBundle,
    manifest: Option<&Path>,
) -> Result<String, Box<dyn Error>> {
    let methodology = normalized(&docs.methodology);

    let question_one_sentence = section_body(&docs.charter, "Question")
        .map(line_like)
        .unwrap_or(false);
    let one_allowed_change = has_field(&docs.charter, "allowed_change");
    let engine_frozen = has_field(&docs.charter, "engine_version");
    let corpus_frozen = has_field(&docs.charter, "case_corpus_version");
    let protocol_frozen = has_field(&docs.charter, "input_protocol_version");
    let provider_model_frozen = has_field(&docs.charter, "provider_selection_version")
        && contains_any(&methodology, &["- provider:", "- model:", "- snapshot:"]);
    let thresholds_frozen = has_field(&docs.charter, "thresholds");
    let primary_metrics_defined = section_body(&docs.methodology, "Metrics")
        .map(|body| contains_any(&normalized(body), &["### primary"]) && bullet_count(body) > 0)
        .unwrap_or(false);
    let stop_condition_defined = section_body(&docs.charter, "Stop condition")
        .map(|body| !body.is_empty())
        .unwrap_or(false);
    let versioning_defined = section_body(&docs.charter, "Versioning rule")
        .map(|body| contains_any(body, &["new_run", "new_cycle", "new_lane", "v2"]))
        .unwrap_or(false);
    let manifest_prepared = manifest.map(manifest_valid).unwrap_or(false);
    let report_schema_defined = has_field(&docs.charter, "report_schema_version")
        || contains_any(&methodology, &["- report_schema_version:"]);
    let raw_outputs_retained = contains_any(&methodology, &["- raw_output_retained: yes"]);
    let ro_crate_present = contains_any(&methodology, &["ro-crate"]);
    let prov_present = contains_any(&methodology, &["prov-aligned", "provenance_style"]);

    let all_checks = [
        question_one_sentence,
        one_allowed_change,
        engine_frozen,
        corpus_frozen,
        protocol_frozen,
        provider_model_frozen,
        thresholds_frozen,
        primary_metrics_defined,
        stop_condition_defined,
        versioning_defined,
        manifest_prepared,
        report_schema_defined,
        raw_outputs_retained,
        ro_crate_present,
        prov_present,
    ];
    let ready = all_checks.into_iter().all(|value| value);

    let mut markdown = String::new();
    writeln!(&mut markdown, "# Pre-Cycle Checklist\n").ok();
    writeln!(&mut markdown, "## Question").ok();
    writeln!(
        &mut markdown,
        "- {} The cycle question fits in one sentence",
        yes_mark(question_one_sentence)
    )
    .ok();

    writeln!(&mut markdown, "\n## Allowed change").ok();
    writeln!(
        &mut markdown,
        "- {} Only one main variable is allowed to change",
        yes_mark(one_allowed_change)
    )
    .ok();

    writeln!(&mut markdown, "\n## Frozen dimensions").ok();
    writeln!(&mut markdown, "- {} Engine frozen", yes_mark(engine_frozen)).ok();
    writeln!(&mut markdown, "- {} Corpus frozen", yes_mark(corpus_frozen)).ok();
    writeln!(
        &mut markdown,
        "- {} Prompt or protocol frozen",
        yes_mark(protocol_frozen)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Provider and model frozen",
        yes_mark(provider_model_frozen)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Thresholds frozen",
        yes_mark(thresholds_frozen)
    )
    .ok();

    writeln!(&mut markdown, "\n## Method").ok();
    writeln!(
        &mut markdown,
        "- {} Primary metrics defined",
        yes_mark(primary_metrics_defined)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Stop condition defined",
        yes_mark(stop_condition_defined)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Versioning rule defined",
        yes_mark(versioning_defined)
    )
    .ok();

    writeln!(&mut markdown, "\n## Output").ok();
    writeln!(
        &mut markdown,
        "- {} Run manifest prepared",
        yes_mark(manifest_prepared)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Report schema defined",
        yes_mark(report_schema_defined)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Raw outputs retained",
        yes_mark(raw_outputs_retained)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} RO-Crate will be generated",
        yes_mark(ro_crate_present)
    )
    .ok();
    writeln!(
        &mut markdown,
        "- {} Minimum PROV-aligned provenance is present",
        yes_mark(prov_present)
    )
    .ok();

    writeln!(&mut markdown, "\n## Go / No-Go").ok();
    writeln!(
        &mut markdown,
        "- {} This cycle is ready to run",
        yes_mark(ready)
    )
    .ok();

    Ok(markdown)
}

#[derive(Debug, Clone)]
struct CanonicalResult {
    label: &'static str,
    present: bool,
    field_used: &'static str,
    notes: String,
}

fn render_canonical_markdown(
    docs: &DocBundle,
    checked_by: &str,
    document_id: &str,
) -> Result<String, Box<dyn Error>> {
    let charter = normalized(&docs.charter);
    let methodology = normalized(&docs.methodology);
    let combined = format!("{charter}\n{methodology}");

    let results = vec![
        CanonicalResult {
            label: "actor / responsible agent",
            present: contains_any(&combined, &["- owner:", "- author:", "- checked_by:"]),
            field_used: "owner / author",
            notes: "Identity fields in charter and methodology".to_owned(),
        },
        CanonicalResult {
            label: "action / intervention",
            present: contains_any(
                &combined,
                &["allowed_change", "comparison_axis", "objective"],
            ),
            field_used: "allowed_change / comparison_axis",
            notes: "The changed variable is named explicitly".to_owned(),
        },
        CanonicalResult {
            label: "object / scope",
            present: contains_any(
                &combined,
                &["## scope", "corpus and lanes", "unit_of_analysis"],
            ),
            field_used: "scope / corpus and lanes / unit_of_analysis",
            notes: "The benchmark object is scoped".to_owned(),
        },
        CanonicalResult {
            label: "temporal or versioned context",
            present: contains_any(
                &combined,
                &[
                    "- date_opened:",
                    "- date:",
                    "engine_version",
                    "methodology_id",
                    "snapshot",
                ],
            ),
            field_used: "date / version / snapshot",
            notes: "Temporal and version context are explicit".to_owned(),
        },
        CanonicalResult {
            label: "evidentiary basis / confirmed_by",
            present: contains_any(
                &combined,
                &[
                    "metrics of interest",
                    "raw_output_retained",
                    "reporting",
                    "reports",
                ],
            ),
            field_used: "metrics / reporting / raw_output_retained",
            notes: "The evidence path is specified even in scientific language".to_owned(),
        },
        CanonicalResult {
            label: "success path / if_ok",
            present: contains_any(
                &combined,
                &["success signal", "improves verdict legitimacy"],
            ),
            field_used: "success signal / interpretation rules",
            notes: "Positive advancement condition is present".to_owned(),
        },
        CanonicalResult {
            label: "inconclusive path / if_doubt",
            present: contains_any(
                &combined,
                &["ghost", "requiresepistemicresolution", "false_ghost_count"],
            ),
            field_used: "ghost metrics / adequacy classes",
            notes: "Suspension and non-closure paths are explicit".to_owned(),
        },
        CanonicalResult {
            label: "failure path / if_not",
            present: contains_any(
                &combined,
                &["failure signal", "false_commit_count", "false_reject_count"],
            ),
            field_used: "failure signal / false_commit_count",
            notes: "Failure conditions are explicit".to_owned(),
        },
        CanonicalResult {
            label: "current status",
            present: contains_any(
                &combined,
                &[
                    "- status: planned",
                    "- status: running",
                    "- status: frozen",
                    "- status: active",
                    "- status: closed",
                    "- status: superseded",
                ],
            ),
            field_used: "status",
            notes: "Current document state is explicit".to_owned(),
        },
    ];

    let present_count = results.iter().filter(|result| result.present).count();
    let status = if present_count == results.len() {
        "PASS"
    } else if present_count >= results.len().saturating_sub(2) {
        "PASS WITH CAVEATS"
    } else {
        "FAIL"
    };
    let missing = results
        .iter()
        .filter(|result| !result.present)
        .map(|result| result.label)
        .collect::<Vec<_>>();

    let mut markdown = String::new();
    writeln!(&mut markdown, "# Canonical Completeness Check\n").ok();
    writeln!(&mut markdown, "## Identity").ok();
    writeln!(&mut markdown, "- template_family: v1").ok();
    writeln!(&mut markdown, "- document_type: cycle package").ok();
    writeln!(&mut markdown, "- document_id: {document_id}").ok();
    writeln!(&mut markdown, "- checked_by: {checked_by}").ok();
    writeln!(&mut markdown, "- date: {}", date_utc()).ok();
    writeln!(&mut markdown, "- status: {status}").ok();

    writeln!(&mut markdown, "\n## Canonical functions present?\n").ok();
    writeln!(
        &mut markdown,
        "| Canonical function | Present? | Scientific field used in this document | Notes |"
    )
    .ok();
    writeln!(&mut markdown, "|---|---|---|---|").ok();
    for result in &results {
        writeln!(
            &mut markdown,
            "| {} | {} | {} | {} |",
            result.label,
            if result.present { "yes" } else { "no" },
            result.field_used,
            result.notes
        )
        .ok();
    }

    writeln!(&mut markdown, "\n## Missing pieces").ok();
    if missing.is_empty() {
        writeln!(
            &mut markdown,
            "- No major structural gaps detected automatically."
        )
        .ok();
    } else {
        for item in missing {
            writeln!(&mut markdown, "- {item}").ok();
        }
    }

    writeln!(&mut markdown, "\n## Decision").ok();
    writeln!(&mut markdown, "- {status}").ok();

    Ok(markdown)
}

fn main() -> Result<(), Box<dyn Error>> {
    let Some(config) = parse_cli().map_err(|err| format!("{err}"))? else {
        println!("{}", usage());
        return Ok(());
    };

    match config {
        CommandConfig::Preflight {
            charter,
            methodology,
            manifest,
            out,
        } => {
            let docs = load_docs(&charter, &methodology)?;
            let content = render_preflight_markdown(&docs, manifest.as_deref())?;
            write_output(&out, &content)?;
            println!("preflight checklist written to {}", out.display());
        }
        CommandConfig::Canonical {
            charter,
            methodology,
            out,
            checked_by,
            document_id,
        } => {
            let docs = load_docs(&charter, &methodology)?;
            let document_id = document_id.unwrap_or_else(|| {
                out.file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "canonical-check".to_owned())
            });
            let content = render_canonical_markdown(&docs, &checked_by, &document_id)?;
            write_output(&out, &content)?;
            println!("canonical check written to {}", out.display());
        }
    }

    Ok(())
}
