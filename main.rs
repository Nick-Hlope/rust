pub struct LinearRegression {
    pub slope:f64,
    pub intercept:f64,
}

fn main() {
    //new instance of the LinearRegression struct
    let mut new_linear_regression = LinearRegression {
        slope: 0.0,
        intercept: 0.0,
    };
}

impl LinearRegression {
    pub fn new() -> LinearRegression {
        LinearRegression {
            slope: 0.0,
            intercept: 0.0,
        }
    }
    pub fn fit (&mut self, input:Vec<f64>, output:Vec<f64>) {
        if input.len()!=output.len() {
            panic!("The number of input and output values is different");
        }
        let size_of_input = input.len();
        let sum_of_input:f64=input.iter().sum::<f64>();
        let sum_of_output:f64=output.iter().sum::<f64>();
        let sum_of_input_output=input.iter().zip(output.iter()).map(|(&x, &y) | x*y).sum::<f64>();
        let square_sum_input=input.iter().map(|&x|x*x).sum::<f64>();

        self.slope=(size_of_input as f64*sum_of_input_output-sum_of_input*sum_of_output)/(size_of_input as f64*square_sum_input-sum_of_input*sum_of_input);
        self.intercept=(sum_of_output*square_sum_input-sum_of_input*sum_of_input_output)/(size_of_input as f64*square_sum_input-sum_of_input*sum_of_input);
    }
}

mod linear_regression;
use linear_regression::LinearRegression;
fn main() {
    let mut a = LinearRegression::new();
    let fahrenheit = vec! [1.,2.,3.,4.,5.,6.,7.,8.,9.];
    let celcius = vec! [-17.22,-16.67,-16.11,-15.56,-15.0,-14.44,-13.89,-13.33,-12.78];
    a.fit(fahrenheit,celsius);

    println!("The slope is {}", a.slope);
    println!("The intercept is {}", a.intercept);
    println!("Celsius as Farenheit{}", a.slope*9.+a.intercept);
}