use anyhow::Result;
use crate::core::config::ProjectConfig;
use crate::core::planner::{TaskDecomposer, ExecutionContext, ExecutionPlan};
use crate::utils::FileUtils;

pub async fn run_task(prompt: &str, dry_run: bool) -> Result<()> {
    let project_root = std::env::current_dir()?;
    let config = ProjectConfig::load(project_root.clone())?;

    tracing::info!("Starting task execution");
    tracing::info!("Prompt: {}", prompt);
    tracing::info!("Dry run mode: {}", dry_run);

    let provider_name = match &config.model_provider {
        crate::core::config::ModelProvider::Ollama { base_url } => format!("Ollama ({})", base_url),
        crate::core::config::ModelProvider::OpenAI { .. } => "OpenAI".to_string(),
        crate::core::config::ModelProvider::Anthropic { .. } => "Anthropic".to_string(),
    };

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║               CodeAgent - Task Execution (Phase 2)           ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║ Model Provider: {:<45} ║", provider_name);
    println!("║ Dry Run Mode:   {:<45} ║", dry_run);
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("Task: {}\n", prompt);

    let plan = TaskDecomposer::decompose_task(prompt)?;
    
    println!("📋 Execution Plan ({} steps):", plan.steps.len());
    println!("   Complexity: {:?}", plan.total_complexity);
    println!("   Estimated Duration: ~{}ms\n", plan.estimated_duration_ms);

    for (i, step) in plan.steps.iter().enumerate() {
        println!("   [Step {}] {} ({})", 
            i + 1, 
            step.description,
            format!("{:?}", step.action_type)
        );
        if !step.dependencies.is_empty() {
            println!("            Depends on: {:?}", step.dependencies);
        }
    }

    println!("\n📁 Project Analysis:");
    let rust_files = FileUtils::find_files(&project_root, "rs")?;
    println!("   Found {} Rust files", rust_files.len());

    if plan.requires_user_approval {
        println!("\n⚠️  This operation requires careful handling.");
        println!("   Recommended: Run with --dry-run first to preview changes");
    }

    let mut context = ExecutionContext::new(plan, dry_run);

    println!("\n🚀 Execution Status:");
    println!("   Steps to Execute: {}", context.plan.steps.len());
    println!("   Progress: {}%", context.progress_percentage());
    
    if dry_run {
        println!("\n🔍 [DRY RUN MODE] - Changes would be applied but not persisted");
        println!("   All modifications are simulated without actual file changes");
    }

    println!("\n✅ Status: Ready for execution\n");

    tracing::info!("Task execution completed");
    Ok(())
}
