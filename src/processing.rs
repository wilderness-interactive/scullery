use crate::data::*;

pub fn processing_steps_for_mix<'a>(larder: &'a Larder, mix_name: &str) -> Vec<&'a Process> {
    let Some(mix) = larder.mixes.iter().find(|m| m.name == mix_name) else {
        return Vec::new();
    };

    larder
        .processes
        .iter()
        .filter(|p| {
            mix.components
                .iter()
                .any(|c| c.ingredient == p.input && c.form == p.output_form)
        })
        .collect()
}

pub fn print_processing_steps(steps: &[&Process]) {
    println!("=== Processing Steps ===\n");
    for (i, step) in steps.iter().enumerate() {
        let method_desc = match &step.method {
            ProcessMethod::MortarAndPestle => "Mortar and pestle".to_owned(),
            ProcessMethod::HandMill => "Hand mill (Zassenhaus)".to_owned(),
            ProcessMethod::Dehydrate { temp_c, hours } => {
                format!("Dehydrate at {}C for {} hours", temp_c, hours)
            }
            ProcessMethod::Crumble => "Crumble by hand".to_owned(),
            ProcessMethod::SliceAndDry => "Slice and dry".to_owned(),
            ProcessMethod::Render => "Render in heavy pot on low heat".to_owned(),
        };
        println!(
            "  {}. {} -> {:?}: {}",
            i + 1,
            step.input,
            step.output_form,
            method_desc
        );
        if let Some(notes) = &step.notes {
            println!("     {}", notes);
        }
    }
    println!();
}
