fn main() {

}
struct MinStack {
    stack: Vec<i32>,
    only_min_stack: Vec<i32>,
}
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            only_min_stack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        // somente joga menores na min_stack
        if self.only_min_stack.is_empty() || *self.only_min_stack.last().unwrap() >= val {
            self.only_min_stack.push(val);
        }
    }
    
    
    fn pop(&mut self) {
       let aux = self.stack.pop();
       // só retirar da stack min se for valor minimo
       if aux.unwrap() == *self.only_min_stack.last().unwrap() {
          self.only_min_stack.pop();
       }
    }
    
    fn top(&self) -> i32 {
        // get ultimo stack comum
        *self.stack.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        // get ultimo stack somente min
        *self.only_min_stack.last().unwrap()
    }
}
