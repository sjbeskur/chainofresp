use std::collections::HashMap;

pub trait MachineRequest{
    fn get_params(&self) -> HashMap<String,String>;
}

pub trait MachineResponse{
    fn get_output(&self) -> HashMap<String,String>;
}


#[derive(Debug,Clone)]
pub struct MachineRequestImpl{
    pub parms: HashMap<String,String>,
}

impl MachineRequest for MachineRequestImpl{
    fn get_params(&self) -> HashMap<String,String>{
        self.parms.clone()
    }
}

#[derive(Debug,Clone)]
pub struct MachineResponsImpl{
    output: HashMap<String,String>,
}

impl MachineResponse for MachineResponsImpl{
    fn get_output(&self) -> HashMap<String,String>{
        self.output.clone()
    }
}


pub trait Machine<T:MachineRequest, U:MachineResponse>{
    fn process(&self, machine_request: T) -> U;
}

pub struct CheckFileTypeMachine{}
pub struct ReadImageMachine{}
pub struct DisplayImageMachine{}

impl Machine<MachineRequestImpl, MachineResponsImpl> for CheckFileTypeMachine{
    fn process(&self, request: MachineRequestImpl) -> MachineResponsImpl {
        let mut resp = MachineResponsImpl{ output: Default::default() };
        let val = request.parms.get("file");
        if val == None { panic!("file is missing "); }

        resp.output.insert("image".into(), "lena.png".into());
        resp.clone()
    }
}

impl Machine<MachineRequestImpl, MachineResponsImpl> for ReadImageMachine{
    fn process(&self, request: MachineRequestImpl) -> MachineResponsImpl {
        let mut resp = MachineResponsImpl{ output: Default::default() };
        let val = request.parms.get("image");
        if val == None { panic!("image is missing"); }

        resp.output.insert("bytes".into(), "[asdfasdfasdfasdfasdfasdfadf]".into());        
        resp.clone()
    }
}

impl Machine<MachineRequestImpl, MachineResponsImpl> for DisplayImageMachine{
    fn process(&self, request: MachineRequestImpl) -> MachineResponsImpl {
        let resp = MachineResponsImpl{ output: Default::default() };
        let val = request.parms.get("bytes");
        if val == None { panic!("missing bytes"); }

        println!("{}", request.parms.get("bytes").unwrap());        

        resp.clone()
    }
}