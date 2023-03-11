use rustamath::polynomial::*;

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands
use clap::{Parser, Subcommand, Args, ArgAction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Evaluate polynomial f(x)
    Eval(EvalArgs),
    /// Plot polynomial f(x)
    Plot(PlotArgs),
    //roots
}

#[derive(Args)]
struct EvalArgs {
    /// x in f(x)
    x: f64,

    /// Coefficients c0, c1, c2...
    #[arg(required = true)]
    coeffs: Vec<f64>,

    /// Verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Args)]
struct PlotArgs {
    /// Start x
    start: f64,

    // FIXME TODO validate end > start
    /// Start x
    end: f64,

    /// Coefficients c0, c1, c2...
    #[arg(required = true)]
    coeffs: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Eval (args) => {
            eval(args.x, &args.coeffs, args.verbose)
        }
        Commands::Plot (args) => {
            plot(&args.coeffs, args.start, args.end)
        }
    }

}

fn eval(x: f64, coeffs: &[f64], verbose: bool) {
    println!("f({x}) = {y}", x=x, y=polynomial_n(x, coeffs));

    if verbose {
        for (i,c) in coeffs.iter().enumerate() {
            print!("{c}*x^{i}", c=c, i=i);
            if i < (coeffs.len() - 1) {
                print!(" + ");
            }
        }
        println!();
    }
}

// https://crates.io/crates/plotters
// https://docs.rs/plotters/latest/plotters/
fn plot(_coeffs: &[f64], _x_start: f64, _x_end: f64) {
}
