const REGISTER_COUNT: usize = 8;

struct VirtualMachine {
    registers: Vec<usize>,
    executable_code: Vec<Box<dyn Instruction>>,
}

struct LoadInstruction {
    destination: usize,
    value: usize,
}

struct AddInstruction {
    destination: usize,
    register_1: usize,
    register_2: usize,
}

impl Instruction for LoadInstruction {
    fn execute(&self, registers: &mut Vec<usize>) {
        println!("LOAD {} {}", self.destination, self.value);
        registers[self.destination] = self.value;
    }
}

impl Instruction for AddInstruction {
    fn execute(&self, registers: &mut Vec<usize>) {
        println!(
            "ADD {} {} {}",
            self.destination, self.register_1, self.register_2
        );

        let r1 = registers[self.register_1];
        let r2 = registers[self.register_2];
        registers[self.destination] = r1 + r2;
    }
}

trait Instruction {
    fn execute(&self, registers: &mut Vec<usize>) {
        println!("TODO: Implement me");
    }
}

fn dump_registers(registers: Vec<usize>) {
    for (i, r) in registers.iter().enumerate() {
        println!("Register {} = [{}]", i, r);
    }
}

fn main() {
    let mut vm = VirtualMachine {
        registers: vec![0; REGISTER_COUNT],
        executable_code: vec![],
    };

    vm.executable_code.push(Box::new(LoadInstruction {
        destination: 0,
        value: 100,
    }));

    vm.executable_code.push(Box::new(LoadInstruction {
        destination: 1,
        value: 75,
    }));

    vm.executable_code.push(Box::new(AddInstruction {
        destination: 2,
        register_1: 0,
        register_2: 1,
    }));

    for i in vm.executable_code {
        i.execute(&mut vm.registers);
    }

    dump_registers(vm.registers);
}
