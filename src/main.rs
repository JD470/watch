mod files;
mod json_struct;
mod keys;
mod parameters;
mod run;

use parameters::*;
use run::*;

fn main(){
    parse_parameters();
    
    run();
}