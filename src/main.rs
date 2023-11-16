use flatbuffers::FlatBufferBuilder;

use crate::cursos_generated::teste::{Base, Any, TypeA, TypeAArgs, BaseArgs};

mod cursos_generated;

fn main() {
    let mut builder = FlatBufferBuilder::new();

    let course_name = builder.create_string("nome generico");
    let teste = TypeA::create(&mut builder, &TypeAArgs{data: 111});


    let course = Base::create(&mut builder, &BaseArgs{ id:1 
        , name: Some(course_name), device: Some(teste.as_union_value()), device_type: Any::TypeA  });

    builder.finish(course, None);

    let buf: &[u8] = builder.finished_data();

    //  DESEREALIZER
     let _msg = cursos_generated::teste::root_as_base(buf);

        let msg = _msg.unwrap();
        match msg.device_type() {
            Any::TypeA => print!("A{:?}",msg.device_as_type_a()),
            Any::TypeB => print!("B{:?}",msg.device_as_type_b()),
            Any::TypeC => print!("C{:?}",msg.device_as_type_c()),
            Any::TypeD => print!("D{:?}",msg.device_as_type_d()),
            x => { print!("{:?}", x)}
        }

    
    

}
