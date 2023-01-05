use chainofresp::*;

fn main() {
    println!("Hello, world!");


    let mut machines : Vec<Box<dyn Machine<MachineRequestImpl, MachineResponsImpl>>> = Vec::new();

    machines.push(Box::new(CheckFileTypeMachine{}));
    machines.push(Box::new(ReadImageMachine{}));
    machines.push(Box::new(DisplayImageMachine{}));

    let mut rq = MachineRequestImpl{ parms: std::collections::HashMap::from([("file".into(),"".into())]), };

    for m in machines{
        let resp = m.process(rq.clone());

        rq = MachineRequestImpl{ parms: Default::default()};
        rq.parms = resp.get_output();


    }

}
