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
    /// Evaluate polynomial f(x) as product of linear factors
    EvalAsFactors(EvalAsFactorsArgs),
    ////TODO  Find roots of polynomial f(x)
    //TOD Roots(RootsArgs),
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
struct EvalAsFactorsArgs {
    /// x in f(x)
    x: f64,

    /// Scale in f(x)=Scale*(x - x0)*(x - x1)...
    scale: f64,

    /// Roots x0, x1, x2...
    #[arg(required = true)]
    roots: Vec<f64>,

    /// Verbose output
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Args)]
struct PlotArgs {
    /// Calculate polynomial as product of linear factors
    #[arg(long, action = ArgAction::SetTrue)]
    as_factors: bool,

    /// Optional scale
    #[arg(long, default_value = "1")]
    scale: f64,

    /// Plot file name w/o extention
    #[arg(short, long)]
    file_name: String,

    /// Start 
    #[arg(short, long)]
    start: f64,

    // FIXME TODO validate end > start
    /// End x
    #[arg(short, long)]
    end: f64,

    /// Derivative
    #[arg(short, long)]
    derivative: Option<f64>,

    /// Coefficients c0, c1, c2... or roots if --as-factors
    #[arg(required = true)]
    coeffs: Vec<f64>,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Eval (args) => {
            eval(args.x, &args.coeffs, args.verbose)
        }
        Commands::EvalAsFactors (args) => {
            eval_as_factors(args.x, args.scale, &args.roots, args.verbose)
        }
        Commands::Plot (args) => {
            if let Err(err) = plot(&args.coeffs, args.start, args.end, args) {
                println!("Error {}", err);
            }
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

fn eval_as_factors(x: f64, scale: f64, roots: &[f64], verbose: bool) {
    println!("f({x}) = {y}", x=x, y=polynomial_as_product_of_linear_factors(x, scale, roots));

    if verbose {
        print!("{}*", scale);
        for (i,root) in roots.iter().enumerate() {
            print!("(x - {})", root);
            if i < (roots.len() - 1) {
                print!("*");
            }
        }
        println!();
    }
}

fn print_formula(coeffs: &[f64], scale: f64, as_factors: bool) -> String {
    let mut s = String::new();
    if as_factors {
        s.push_str(&format!("{}*", scale));
        for (i,root) in coeffs.iter().enumerate() {
            s.push_str(&format!("(x - {})", root));
            if i < (coeffs.len() - 1) {
                s.push('*');
            }
        }
    }
    else {
        for (i,c) in coeffs.iter().enumerate() {
            s.push_str(&format!("{c}*x^{i}", c=c, i=i));
            if i < (coeffs.len() - 1) {
                s.push_str(" + ");
            }
        }
    }
    s
}

use plotters::prelude::*;

/*const plot_dot_and_label: _ = |x: f64, y: f64| {
    return EmptyElement::at((x, y))
        + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
        + Text::new(
            format!("({:.2},{:.2})", x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
};*/

// https://crates.io/crates/plotters
// https://docs.rs/plotters/latest/plotters/
// `rustamath-polynomial plot -f ../plot -s=-10 -e 10 -- 8 1 1 -1`
fn plot(coeffs: &[f64], x_start: f64, x_end: f64, args: &PlotArgs)
-> Result<(), Box<dyn std::error::Error>>
{
    let file_name = String::from(&args.file_name) + ".svg";
    println!("Saving to file {}", file_name);
    let backend = SVGBackend::new(
        &file_name,
        (800, 800)).into_drawing_area();

    backend.fill(&WHITE)?;

    let poly_fun = |x| {
        if args.as_factors {
            polynomial_as_product_of_linear_factors(x, args.scale, coeffs)
        }
        else {
            polynomial_n(x, coeffs) * args.scale
        }
    };

    let ps: _ = (0..=100)
        .map(|x| x_start + (x as f64)*(x_end - x_start)/100.0)
        //.map(|x| (x, polynomial_n(x, coeffs)))
        .map(|x| (x, poly_fun(x)))
        .collect::<Vec<(f64,f64)>>();
    let y_start = ps.iter().fold(f64::INFINITY, |a, (_x,y)| a.min(*y));
    let y_end = ps.iter().fold(f64::NEG_INFINITY, |a, (_x,y)| a.max(*y));

    println!("ranges x:[{} .. {}] y:[{} .. {}]", x_start, x_end, y_start, y_end);

    let mut chart = ChartBuilder::on(&backend)
        .caption(print_formula(coeffs, args.scale, args.as_factors),
            ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(x_start..x_end, y_start..y_end)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            ps,
            &RED,
        ))?
        .label("y = P(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
    
    if let Some(dp_x) = args.derivative {
        let (_, p_y, dp) = derivative_polynomial_n(dp_x, coeffs);
        let offset = p_y - dp*dp_x;
        let dps: _ = (0..=100)
            .map(|x| x_start + (x as f64)*(x_end - x_start)/100.0)
            .map(|x| (x, offset + dp*x))
            .collect::<Vec<(f64,f64)>>();

        chart
            .draw_series(LineSeries::new(
                dps,
                &BLUE,
            ))?
            .label("y = dP(x)/dx")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));
        
        /*let dot_and_label = |x: f32, y: f32| {
            return EmptyElement::at((x, y))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                + Text::new(
                    format!("({:.2},{:.2})", x, y),
                    (10, 0),
                    ("sans-serif", 15.0).into_font(),
                );
        };

        backend.draw(&dot_and_label(dp_x as f32, p_y as f32))?;*/
    }

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}
