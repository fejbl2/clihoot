use client::terminal::student::run_student;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let (term, task) = run_student().await?;

    task.await??;

    Ok(())
}
