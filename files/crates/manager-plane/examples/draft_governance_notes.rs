use manager_plane::VersionedBenchmarkReport;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandConfig {
    Comparison {
        runs: Vec<PathBuf>,
        out: PathBuf,
        note_id: Option<String>,
        author: String,
    },
    Decision {
        cycle_id: String,
        reports: Vec<PathBuf>,
        notes: Vec<PathBuf>,
        out: PathBuf,
        decision_id: Option<String>,
        author: String,
    },
}

fn usage() -> &'static str {
    "usage:
  cargo run -p manager-plane --example draft_governance_notes -- comparison \
    --run /path/to/run-a.json --run /path/to/run-b.json [--run /path/to/run-c.json ...] \
    --out /path/to/COMPARATIVE_NOTE.md [--note-id cycle-2-none-vs-medium] [--author codex]

  cargo run -p manager-plane --example draft_governance_notes -- decision \
    --cycle-id cycle-2 \
    --report /path/to/report-a.json --report /path/to/report-b.json [--report /path/to/report-c.json ...] \
    [--note /path/to/note-a.md ...] \
    --out /path/to/DECISION_NOTE.md [--decision-id cycle-2-freeze] [--author codex]"
}

fn parse_cli() -> Result<Option<CommandConfig>, String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage().to_owned());
    };
    if matches!(command.as_str(), "--help" | "-h") {
        return Ok(None);
    }

    let mut runs = Vec::new();
    let mut reports = Vec::new();
    let mut notes = Vec::new();
    let mut out = None;
    let mut note_id = None;
    let mut decision_id = None;
    let mut author = Some("codex".to_owned());
    let mut cycle_id = None;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--run" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--run requires a path".to_owned())?;
                runs.push(PathBuf::from(value));
            }
            "--report" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--report requires a path".to_owned())?;
                reports.push(PathBuf::from(value));
            }
            "--note" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--note requires a path".to_owned())?;
                notes.push(PathBuf::from(value));
            }
            "--out" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--out requires a path".to_owned())?;
                out = Some(PathBuf::from(value));
            }
            "--note-id" => {
                note_id = Some(
                    args.next()
                        .ok_or_else(|| "--note-id requires a value".to_owned())?,
                );
            }
            "--decision-id" => {
                decision_id = Some(
                    args.next()
                        .ok_or_else(|| "--decision-id requires a value".to_owned())?,
                );
            }
            "--author" => {
                author = Some(
                    args.next()
                        .ok_or_else(|| "--author requires a value".to_owned())?,
                );
            }
            "--cycle-id" => {
                cycle_id = Some(
                    args.next()
                        .ok_or_else(|| "--cycle-id requires a value".to_owned())?,
                );
            }
            "--help" | "-h" => return Ok(None),
            other => return Err(format!("unknown flag `{other}`\n\n{}", usage())),
        }
    }

    let out = out.ok_or_else(|| format!("missing --out\n\n{}", usage()))?;
    let author = author.unwrap_or_else(|| "codex".to_owned());

    match command.as_str() {
        "comparison" => {
            if runs.len() < 2 {
                return Err(format!(
                    "comparison requires at least two --run arguments\n\n{}",
                    usage()
                ));
            }
            Ok(Some(CommandConfig::Comparison {
                runs,
                out,
                note_id,
                author,
            }))
        }
        "decision" => {
            if reports.is_empty() {
                return Err(format!(
                    "decision requires at least one --report argument\n\n{}",
                    usage()
                ));
            }
            let cycle_id = cycle_id.ok_or_else(|| format!("missing --cycle-id\n\n{}", usage()))?;
            Ok(Some(CommandConfig::Decision {
                cycle_id,
                reports,
                notes,
                out,
                decision_id,
                author,
            }))
        }
        other => Err(format!(
            "unknown command `{other}`; expected `comparison` or `decision`\n\n{}",
            usage()
        )),
    }
}

#[derive(Debug, Clone)]
struct LoadedReport {
    path: PathBuf,
    report: VersionedBenchmarkReport,
}

fn load_versioned_report(path: &Path) -> Result<LoadedReport, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let report = serde_json::from_str::<VersionedBenchmarkReport>(&raw)?;
    Ok(LoadedReport {
        path: path.to_path_buf(),
        report,
    })
}

fn lane_key(report: &LoadedReport) -> String {
    report
        .path
        .parent()
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| {
            report
                .report
                .run
                .case_corpus
                .label
                .clone()
                .unwrap_or_default()
        })
}

fn reasoning_rank(value: Option<&str>) -> usize {
    match value.unwrap_or("") {
        "none" => 0,
        "low" => 1,
        "medium" => 2,
        "high" => 3,
        "xhigh" => 4,
        _ => 99,
    }
}

fn sort_by_reasoning(reports: &mut [LoadedReport]) {
    reports.sort_by_key(|loaded| {
        reasoning_rank(loaded.report.run.provider.reasoning_effort.as_deref())
    });
}

fn duration_seconds(report: &LoadedReport) -> u64 {
    let timing = &report.report.run.timing;
    timing
        .completed_at_unix_s
        .saturating_sub(timing.started_at_unix_s)
}

fn note_id_default(out: &Path) -> String {
    out.file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| "comparative-note".to_owned())
}

fn decision_id_default(cycle_id: &str) -> String {
    format!("{cycle_id}-decision")
}

fn today_utc() -> String {
    let now = OffsetDateTime::now_utc().date();
    format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    )
}

fn render_comparison_markdown(
    reports: &[LoadedReport],
    note_id: &str,
    author: &str,
) -> Result<String, Box<dyn Error>> {
    let mut groups: BTreeMap<String, Vec<LoadedReport>> = BTreeMap::new();
    for report in reports {
        groups
            .entry(lane_key(report))
            .or_default()
            .push(report.clone());
    }
    for group in groups.values_mut() {
        sort_by_reasoning(group);
    }

    let mut engine_values = BTreeSet::new();
    let mut adapter_values = BTreeSet::new();
    let mut spec_values = BTreeSet::new();
    let mut backend_values = BTreeSet::new();
    let mut model_values = BTreeSet::new();
    let mut case_corpus_values = BTreeSet::new();
    let mut reasoning_values = BTreeSet::new();

    for report in reports {
        engine_values.insert(report.report.run.engine.auto_revision.clone());
        adapter_values.insert(report.report.run.adapter.auto_revision.clone());
        spec_values.insert(report.report.run.benchmark_spec.auto_revision.clone());
        backend_values.insert(report.report.run.provider.backend.clone());
        model_values.insert(
            report
                .report
                .run
                .provider
                .model
                .clone()
                .unwrap_or_else(|| "unknown".to_owned()),
        );
        case_corpus_values.insert(report.report.run.case_corpus.auto_revision.clone());
        reasoning_values.insert(
            report
                .report
                .run
                .provider
                .reasoning_effort
                .clone()
                .unwrap_or_else(|| "unspecified".to_owned()),
        );
    }

    let held_constant = [
        ("engine", engine_values.len() == 1),
        ("adapter", adapter_values.len() == 1),
        ("benchmark protocol", spec_values.len() == 1),
        ("provider backend", backend_values.len() == 1),
        ("model snapshot", model_values.len() == 1),
    ]
    .into_iter()
    .filter_map(|(label, keep)| keep.then_some(label))
    .collect::<Vec<_>>();

    let mut main_result =
        "The compared runs changed output style more than governed legitimacy.".to_owned();
    let mut improvements = Vec::new();
    let mut worsened = Vec::new();
    let mut unchanged = Vec::new();
    let mut case_differences = Vec::new();
    let mut metric_tables = String::new();

    for (lane, group) in &groups {
        if group.len() < 2 {
            continue;
        }
        let a = &group[0];
        let b = &group[1];
        let label_a = a
            .report
            .run
            .provider
            .reasoning_effort
            .clone()
            .unwrap_or_else(|| "run-a".to_owned());
        let label_b = b
            .report
            .run
            .provider
            .reasoning_effort
            .clone()
            .unwrap_or_else(|| "run-b".to_owned());
        let metrics_a = &a.report.report.metrics;
        let metrics_b = &b.report.report.metrics;
        writeln!(&mut metric_tables, "\n### `{lane}`\n").ok();
        writeln!(
            &mut metric_tables,
            "| Metric | {label_a} | {label_b} | Difference | Interpretation |"
        )
        .ok();
        writeln!(&mut metric_tables, "|---|---:|---:|---:|---|").ok();
        for (name, av, bv, interpretation) in [
            (
                "correct_commit_count",
                metrics_a.correct_commit_count as isize,
                metrics_b.correct_commit_count as isize,
                "Lawful commit behavior",
            ),
            (
                "correct_ghost_count",
                metrics_a.correct_ghost_count as isize,
                metrics_b.correct_ghost_count as isize,
                "Recoverable insufficiency handling",
            ),
            (
                "correct_reject_count",
                metrics_a.correct_reject_count as isize,
                metrics_b.correct_reject_count as isize,
                "Illegitimate closure containment",
            ),
            (
                "false_commit_count",
                metrics_a.false_commit_count as isize,
                metrics_b.false_commit_count as isize,
                "Safety-critical promotion error",
            ),
            (
                "false_ghost_count",
                metrics_a.false_ghost_count as isize,
                metrics_b.false_ghost_count as isize,
                "Over-blocking or missed lawful closure",
            ),
            (
                "false_reject_count",
                metrics_a.false_reject_count as isize,
                metrics_b.false_reject_count as isize,
                "Improper hard rejection",
            ),
            (
                "transition_correct_count",
                metrics_a.transition_correct_count as isize,
                metrics_b.transition_correct_count as isize,
                "Post-resolution correctness",
            ),
        ] {
            writeln!(
                &mut metric_tables,
                "| {name} | {av} | {bv} | {:+} | {interpretation} |",
                bv - av
            )
            .ok();
        }

        let duration_a = duration_seconds(a);
        let duration_b = duration_seconds(b);
        if metrics_b.false_commit_count > metrics_a.false_commit_count {
            main_result =
                "The later run introduced a false commit and worsened governed legitimacy."
                    .to_owned();
            worsened.push(format!(
                "`{lane}` introduced or increased false commits ({} -> {})",
                metrics_a.false_commit_count, metrics_b.false_commit_count
            ));
        } else if metrics_b.correct_reject_count > metrics_a.correct_reject_count
            || metrics_b.false_ghost_count < metrics_a.false_ghost_count
        {
            improvements.push(format!(
                "`{lane}` improved legitimacy metrics (`correct_reject` {} -> {}, `false_ghost` {} -> {})",
                metrics_a.correct_reject_count,
                metrics_b.correct_reject_count,
                metrics_a.false_ghost_count,
                metrics_b.false_ghost_count
            ));
        } else if metrics_b == metrics_a {
            unchanged.push(format!(
                "`{lane}` kept the same verdict profile while duration changed from {}s to {}s",
                duration_a, duration_b
            ));
        } else {
            worsened.push(format!(
                "`{lane}` changed metrics without a clear legitimacy gain (`false_ghost` {} -> {}, duration {}s -> {}s)",
                metrics_a.false_ghost_count, metrics_b.false_ghost_count, duration_a, duration_b
            ));
        }

        let cases_a: BTreeMap<_, _> = a
            .report
            .report
            .cases
            .iter()
            .map(|case| (case.id.clone(), case))
            .collect();
        let cases_b: BTreeMap<_, _> = b
            .report
            .report
            .cases
            .iter()
            .map(|case| (case.id.clone(), case))
            .collect();
        let mut case_ids: Vec<_> = cases_a.keys().cloned().collect();
        case_ids.sort();
        for case_id in case_ids {
            let Some(case_a) = cases_a.get(&case_id) else {
                continue;
            };
            let Some(case_b) = cases_b.get(&case_id) else {
                continue;
            };
            if case_a.initial_adequacy != case_b.initial_adequacy
                || case_a.resolved.as_ref().map(|resolved| &resolved.adequacy)
                    != case_b.resolved.as_ref().map(|resolved| &resolved.adequacy)
            {
                case_differences.push(format!(
                    "- `{lane}` / `{case_id}`\n  - Run A: `{:?}`\n  - Run B: `{:?}`\n  - Why it matters: outcome changed under a frozen setup.",
                    case_a.initial_adequacy, case_b.initial_adequacy
                ));
            }
        }
    }

    if improvements.is_empty() && worsened.is_empty() && !unchanged.is_empty() {
        main_result =
            "The compared runs converged on the same governed outcomes; the main difference was cost and elaboration."
                .to_owned();
    }

    let mut markdown = String::new();
    writeln!(&mut markdown, "# Comparative Note\n").ok();
    writeln!(&mut markdown, "## Identity").ok();
    writeln!(&mut markdown, "- template_family: v1").ok();
    writeln!(&mut markdown, "- note_id: {note_id}").ok();
    writeln!(&mut markdown, "- date: {}", today_utc()).ok();
    writeln!(&mut markdown, "- author: {author}").ok();
    writeln!(&mut markdown, "- compares:").ok();
    for report in reports {
        writeln!(&mut markdown, "  - {}", report.path.display()).ok();
    }

    writeln!(&mut markdown, "\n## What was held constant").ok();
    for item in &held_constant {
        writeln!(&mut markdown, "- {item}").ok();
    }
    if held_constant.is_empty() {
        writeln!(
            &mut markdown,
            "- no stable dimensions detected automatically"
        )
        .ok();
    }

    writeln!(&mut markdown, "\n## What changed").ok();
    if reasoning_values.len() > 1 {
        writeln!(
            &mut markdown,
            "- reasoning_effort: {}",
            reasoning_values
                .into_iter()
                .collect::<Vec<_>>()
                .join(" vs ")
        )
        .ok();
    } else {
        writeln!(&mut markdown, "- no major change detected automatically").ok();
    }
    if case_corpus_values.len() > 1 {
        writeln!(
            &mut markdown,
            "- lane corpus varied across the compared runs, but remained frozen within each lane pair"
        )
        .ok();
    }

    writeln!(&mut markdown, "\n## Main result").ok();
    writeln!(&mut markdown, "> {main_result}").ok();

    writeln!(&mut markdown, "\n## Metrics comparison").ok();
    markdown.push_str(&metric_tables);

    writeln!(&mut markdown, "\n## Case-by-case differences").ok();
    if case_differences.is_empty() {
        writeln!(
            &mut markdown,
            "- No case-level verdict differences were detected automatically."
        )
        .ok();
    } else {
        for diff in case_differences {
            writeln!(&mut markdown, "{diff}").ok();
        }
    }

    writeln!(&mut markdown, "\n## Honest reading").ok();
    writeln!(&mut markdown, "\n### What improved").ok();
    if improvements.is_empty() {
        writeln!(
            &mut markdown,
            "- No clear legitimacy improvement was detected automatically."
        )
        .ok();
    } else {
        for item in improvements {
            writeln!(&mut markdown, "- {item}").ok();
        }
    }
    writeln!(&mut markdown, "\n### What got worse").ok();
    if worsened.is_empty() {
        writeln!(
            &mut markdown,
            "- No automatic regression signal was detected."
        )
        .ok();
    } else {
        for item in worsened {
            writeln!(&mut markdown, "- {item}").ok();
        }
    }
    writeln!(&mut markdown, "\n### What stayed unchanged").ok();
    if unchanged.is_empty() {
        writeln!(
            &mut markdown,
            "- No unchanged lanes were detected automatically."
        )
        .ok();
    } else {
        for item in unchanged {
            writeln!(&mut markdown, "- {item}").ok();
        }
    }

    writeln!(&mut markdown, "\n## Methodological conclusion").ok();
    writeln!(
        &mut markdown,
        "> Draft conclusion: interpret legitimacy changes before stylistic richness or proposer verbosity."
    )
    .ok();
    writeln!(&mut markdown, "\n## What it does NOT prove").ok();
    writeln!(
        &mut markdown,
        "> Draft caution: this comparison does not by itself prove that the changed variable can never help under a different corpus, protocol, or model family."
    )
    .ok();
    writeln!(&mut markdown, "\n## Next step").ok();
    writeln!(
        &mut markdown,
        "- Human review: confirm whether this draft should be frozen as a cycle-level comparative note or refined lane by lane."
    )
    .ok();

    Ok(markdown)
}

fn render_decision_markdown(
    cycle_id: &str,
    reports: &[LoadedReport],
    notes: &[PathBuf],
    decision_id: &str,
    author: &str,
) -> Result<String, Box<dyn Error>> {
    let mut groups: BTreeMap<String, Vec<LoadedReport>> = BTreeMap::new();
    for report in reports {
        groups
            .entry(lane_key(report))
            .or_default()
            .push(report.clone());
    }
    for group in groups.values_mut() {
        sort_by_reasoning(group);
    }

    let mut freeze_reasons = Vec::new();
    let mut open_questions = Vec::new();
    let mut reports_list = reports
        .iter()
        .map(|report| format!("  - `{}`", report.path.display()))
        .collect::<Vec<_>>();
    reports_list.sort();

    let mut note_list = notes
        .iter()
        .map(|path| format!("  - `{}`", path.display()))
        .collect::<Vec<_>>();
    note_list.sort();

    let mut any_improvement = false;
    let mut any_false_commit_regression = false;
    let mut any_cost_increase = false;

    for (lane, group) in &groups {
        if group.len() < 2 {
            open_questions.push(format!(
                "`{lane}` does not yet have a complete pair of reports for automated freeze analysis"
            ));
            continue;
        }
        let a = &group[0];
        let b = &group[1];
        let ma = &a.report.report.metrics;
        let mb = &b.report.report.metrics;
        if mb.false_commit_count > ma.false_commit_count {
            any_false_commit_regression = true;
            freeze_reasons.push(format!(
                "`{lane}` introduced a false commit under higher reasoning effort ({} -> {})",
                ma.false_commit_count, mb.false_commit_count
            ));
        }
        if mb.correct_reject_count > ma.correct_reject_count
            || mb.false_ghost_count < ma.false_ghost_count
            || mb.transition_correct_count > ma.transition_correct_count
        {
            any_improvement = true;
            freeze_reasons.push(format!(
                "`{lane}` showed some legitimacy improvement and may need explicit human interpretation before freezing"
            ));
        }
        let duration_a = duration_seconds(a);
        let duration_b = duration_seconds(b);
        if duration_b > duration_a {
            any_cost_increase = true;
        }
    }

    let decision = if any_false_commit_regression || !any_improvement {
        "Freeze the cycle and stop experimentation on this axis for now."
    } else {
        "Human review required before freezing; at least one lane shows an improvement signal."
    };

    let why_now = if any_false_commit_regression {
        "The compared runs answered the cycle question and exposed at least one safety-critical regression under the changed setting."
    } else if !any_improvement {
        "The compared runs answered the cycle question and showed no legitimacy gain under the changed setting."
    } else {
        "The compared runs are complete, but at least one lane needs explicit human interpretation before closing."
    };

    let mut markdown = String::new();
    writeln!(&mut markdown, "# Decision Note\n").ok();
    writeln!(&mut markdown, "## Identity").ok();
    writeln!(&mut markdown, "- template_family: v1").ok();
    writeln!(&mut markdown, "- decision_id: {decision_id}").ok();
    writeln!(&mut markdown, "- date: {}", today_utc()).ok();
    writeln!(&mut markdown, "- author: {author}").ok();
    writeln!(&mut markdown, "- related_cycle: {cycle_id}").ok();

    writeln!(&mut markdown, "\n## Decision").ok();
    writeln!(&mut markdown, "> {decision}").ok();
    if any_false_commit_regression || !any_improvement {
        writeln!(&mut markdown, "\n- freeze_cycle").ok();
        writeln!(&mut markdown, "- stop_experimentation_on_this_axis").ok();
    } else {
        writeln!(&mut markdown, "\n- other: human review required").ok();
    }

    writeln!(&mut markdown, "\n## Why now").ok();
    writeln!(&mut markdown, "> {why_now}").ok();

    writeln!(&mut markdown, "\n## Evidence used").ok();
    writeln!(&mut markdown, "- reports:").ok();
    for line in reports_list {
        writeln!(&mut markdown, "{line}").ok();
    }
    writeln!(&mut markdown, "- notes:").ok();
    if note_list.is_empty() {
        writeln!(&mut markdown, "  - none supplied").ok();
    } else {
        for line in note_list {
            writeln!(&mut markdown, "{line}").ok();
        }
    }

    writeln!(&mut markdown, "\n## What is now frozen").ok();
    writeln!(
        &mut markdown,
        "- the cycle question for `{cycle_id}` under the currently supplied reports"
    )
    .ok();
    writeln!(
        &mut markdown,
        "- the compared lane corpora and report set used to answer this question"
    )
    .ok();
    if any_false_commit_regression {
        writeln!(
            &mut markdown,
            "- the reading that at least one compared setting introduced a false commit and must not be reinterpreted away"
        )
        .ok();
    } else {
        writeln!(
            &mut markdown,
            "- the reading that the changed setting did not produce an automatic legitimacy gain"
        )
        .ok();
    }

    writeln!(&mut markdown, "\n## What remains open").ok();
    if open_questions.is_empty() {
        writeln!(
            &mut markdown,
            "- whether a future cycle should change a different axis instead of repeating the same one"
        )
        .ok();
        writeln!(
            &mut markdown,
            "- whether stronger automation should draft these notes directly by default"
        )
        .ok();
    } else {
        for item in open_questions {
            writeln!(&mut markdown, "- {item}").ok();
        }
    }

    writeln!(&mut markdown, "\n## What comes next").ok();
    writeln!(
        &mut markdown,
        "- next_cycle: define a new question before new runs"
    )
    .ok();
    writeln!(
        &mut markdown,
        "- next_lane: none inside the frozen cycle unless a new decision note reopens it"
    )
    .ok();
    if any_cost_increase {
        writeln!(
            &mut markdown,
            "- next_hypothesis: test a new axis that has a better chance of improving legitimacy without the same cost increase"
        )
        .ok();
    } else {
        writeln!(
            &mut markdown,
            "- next_hypothesis: choose a genuinely new experimental axis"
        )
        .ok();
    }

    writeln!(&mut markdown, "\n## Anti-chaos note").ok();
    writeln!(
        &mut markdown,
        "> Draft guardrail: do not rewrite supplied corpora or reinterpret the compared reports while keeping the same cycle identity."
    )
    .ok();
    writeln!(
        &mut markdown,
        "\n- Do not reopen this cycle without an explicit new charter or decision note."
    )
    .ok();
    writeln!(
        &mut markdown,
        "- Do not relabel safety regressions as stylistic differences."
    )
    .ok();

    Ok(markdown)
}

fn write_output(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let Some(config) = parse_cli().map_err(|err| format!("{err}"))? else {
        println!("{}", usage());
        return Ok(());
    };

    match config {
        CommandConfig::Comparison {
            runs,
            out,
            note_id,
            author,
        } => {
            let mut loaded = runs
                .iter()
                .map(|path| load_versioned_report(path))
                .collect::<Result<Vec<_>, _>>()?;
            loaded.sort_by(|a, b| a.path.cmp(&b.path));
            let note_id = note_id.unwrap_or_else(|| note_id_default(&out));
            let content = render_comparison_markdown(&loaded, &note_id, &author)?;
            write_output(&out, &content)?;
            println!("comparison draft written to {}", out.display());
        }
        CommandConfig::Decision {
            cycle_id,
            reports,
            notes,
            out,
            decision_id,
            author,
        } => {
            let mut loaded = reports
                .iter()
                .map(|path| load_versioned_report(path))
                .collect::<Result<Vec<_>, _>>()?;
            loaded.sort_by(|a, b| a.path.cmp(&b.path));
            let decision_id = decision_id.unwrap_or_else(|| decision_id_default(&cycle_id));
            let content =
                render_decision_markdown(&cycle_id, &loaded, &notes, &decision_id, &author)?;
            write_output(&out, &content)?;
            println!("decision draft written to {}", out.display());
        }
    }

    Ok(())
}
