trait Graph {
	type N;
	type E;
	fn hasEdge(&self,&Self::N,&Self::N)->bool;
	fn edges(&self,&Self::N)->Vec<Self::E>;
}

#[derive(Debug)]
struct Node;
#[derive(Debug)]
struct Edge;
#[derive(Debug)]
struct MyGraph;

impl<'a> Graph for &'a mut MyGraph{
	type N=Node;
	type E=Edge;
	fn hasEdge(&self,n1:&Node,n2:&Node)->bool{
		true
	}
	fn edges(&self,n:&Node)->Vec<Edge>{
		Vec::new()
	}
}

fn main() {
	let mut graph = MyGraph;
	// let g1=Box::new(&mut graph);
	let g2:Box<Graph<N=Node,E=Edge>>=Box::new(&mut graph);
	// let g1=Box::new(&graph) as Box<Graph<N=Node,E=Edge>>;
	
	// let g3=Box::new(graph);
	// println!("{:?}", graph);
}