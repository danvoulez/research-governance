# Methodology Spec

## Identity
- template_family: v1
- methodology_id:
- related_experiment_id:
- author:
- date:
- status: active | frozen | superseded

## Objective
> What does this methodology operationalize?

## Experimental design
- design_type: baseline comparison | adversarial lane | ablation | robustness | other
- unit_of_analysis:
- comparison_axis:

## Inputs
### Case corpus
- corpus_id:
- source:
- lane(s):
- expected_verdict_classes:

### Input protocol
- protocol_id:
- format:
- constraints:
- normalization_rules:

## System configuration
### Engine
- engine_version:
- relevant_modules:
- thresholds_policies:

### Provider and model
- provider:
- model:
- snapshot:
- reasoning_effort:
- other_parameters:

### Adapter
- adapter_version:
- structured_output_requirement:
- parse_rules:
- adequacy_rules:

## Run procedure
1.
2.
3.
4.

## Metrics
### Primary
- false_commit_count
- false_ghost_count
- false_reject_count
- correct_commit_count
- correct_ghost_count
- correct_reject_count
- transition_correct_count

### Secondary
- ghost_utility
- resolution_to_commit_lift
- runtime_latency
- cost
- parse_failure_rate

## Reporting
- report_schema_version:
- output_paths:
- raw_output_retained: yes | no
- research_package_format: RO-Crate
- provenance_style: PROV-aligned partial fields
- software_metadata: CodeMeta

## Interpretation rules
### Examples
- If X happens, interpret as:
- If Y happens, interpret as:
- If Z happens, do not claim:

## Limitations
- 
