use crate::runtime::data::{ Array, Class, Data, FunctionCode, FunctionPrimitive, GenericCode, GenericPrimitive, Method, Nullable, Object, Tag };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::utilities::{ Arguments, Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;

pub type GcValue<'a> = GcRef<Value<'a>>;

pub struct Value<'a> {
    pub class: GcValue<'a>,
    data: Data<'a>,
}

impl<'a> Value<'a> {
    pub fn new(class: GcValue<'a>, data: Data<'a>) -> Self {
        Self {
            class,
            data,
        }
    }
}

impl<'a> GcValue<'a> {
    pub fn is(self, other: GcValue<'a>) -> bool {
        if self == other {
            true
        } else if let Some(parent) = self.data_class().parent {
            parent.is(other)
        } else {
            false
        }
    }

    pub fn isa(self, other: GcValue<'a>) -> bool {
        self.class.is(other)
    }

    pub fn cast(self, other: GcValue<'a>) -> Return<()> {
        if self.isa(other) {
            Ok(())
        } else {
            Err(Error::new_cast(self, other))
        }
    }
}

impl<'a> GcValue<'a> {
    pub fn get_method(&self, name: &str) -> ReturnValue<'a> {
        if let Some(method) = self.class.data_class().get_method(name) {
            Ok(method)
        } else {
            Err(Error::new_undefined_method(name, self.class))
        }
    }

    pub fn call_method(self, engine: &mut Engine<'a>, name: &str, arguments: Arguments<'a>) -> ReturnReference<'a> {
        let mut arguments = arguments.into_vec();
        arguments.insert(0, self);
        self.call_method_self(engine, name, arguments.into_boxed_slice())
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, name: &str, arguments: Arguments<'a>) -> ReturnReference<'a> {
        let method = self.get_method(name)?;
        let array = parameters::pack(engine, arguments);
        method.get_method("__cl__")?.data_function_primitive().call(engine, Box::new([method, array]))
    }

    pub fn call_to_string(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "to_string", Box::new([]))?.read()?.data_string().clone())
    }
}

impl<'a> GcValue<'a> {
    pub fn get_cast_array(&self, engine: &Engine<'a>) -> Return<&Array<'a>> {
        self.cast(engine.primitives.array)?;
        Ok(&self.data_array())
    }

    pub fn get_cast_boolean(&self, engine: &Engine<'a>) -> Return<&bool> {
        self.cast(engine.primitives.boolean)?;
        Ok(self.data_boolean())
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
        self.data.trace();
    }
}

macro_rules! data {
    ( $this:expr, $variant:ident ) => {
        if let Data::$variant(variant) = &$this.data {
            return variant;
        }

        panic!();
    };
}

macro_rules! data_mut {
    ( $this:expr, $variant:ident ) => {
        if let Data::$variant(variant) = &mut $this.data {
            return variant;
        }

        panic!();
    };
}

impl<'a> Value<'a> {
    pub fn data_tag(&self) -> Tag {
        match &self.data {
            Data::Class(class)                => class.tag.clone(),
            Data::FunctionCode(function)      => function.tag.clone(),
            Data::FunctionPrimitive(function) => function.tag.clone(),
            Data::GenericCode(generic)        => generic.tag.clone(),
            Data::GenericPrimitive(generic)   => generic.tag.clone(),
            _ => panic!(),
        }
    }

    pub fn data_array(&self) -> &Array<'a> {
        data!(self, Array);
    }

    pub fn data_array_mut(&mut self) -> &mut Array<'a> {
        data_mut!(self, Array);
    }

    pub fn data_boolean(&self) -> &bool {
        data!(self, Boolean);
    }

    pub fn data_boolean_mut(&mut self) -> &mut bool {
        data_mut!(self, Boolean);
    }

    pub fn data_class(&self) -> &Class<'a> {
        data!(self, Class);
    }

    pub fn data_class_mut(&mut self) -> &mut Class<'a> {
        data_mut!(self, Class);
    }

    pub fn data_function(&self) -> &FunctionCode<'a> {
        data!(self, FunctionCode);
    }

    pub fn data_function_mut(&mut self) -> &mut FunctionCode<'a> {
        data_mut!(self, FunctionCode);
    }

    pub fn data_function_primitive(&self) -> &FunctionPrimitive<'a> {
        data!(self, FunctionPrimitive);
    }

    pub fn data_function_primitive_mut(&mut self) -> &mut FunctionPrimitive<'a> {
        data_mut!(self, FunctionPrimitive);
    }

    pub fn data_generic(&self) -> &GenericCode<'a> {
        data!(self, GenericCode);
    }

    pub fn data_generic_mut(&mut self) -> &mut GenericCode<'a> {
        data_mut!(self, GenericCode);
    }

    pub fn data_generic_primitive(&self) -> &GenericPrimitive<'a> {
        data!(self, GenericPrimitive);
    }

    pub fn data_generic_primitive_mut(&mut self) -> &mut GenericPrimitive<'a> {
        data_mut!(self, GenericPrimitive);
    }

    pub fn data_integer(&self) -> &isize {
        data!(self, Integer);
    }

    pub fn data_integer_mut(&mut self) -> &mut isize {
        data_mut!(self, Integer);
    }

    pub fn data_method(&self) -> &Method<'a> {
        data!(self, Method);
    }

    pub fn data_method_mut(&mut self) -> &mut Method<'a> {
        data_mut!(self, Method);
    }

    pub fn data_object(&self) -> &Object<'a> {
        data!(self, Object);
    }

    pub fn data_object_mut(&mut self) -> &mut Object<'a> {
        data_mut!(self, Object);
    }

    pub fn data_nullable(&self) -> &Nullable<'a> {
        data!(self, Nullable);
    }

    pub fn data_nullable_mut(&mut self) -> &mut Nullable<'a> {
        data_mut!(self, Nullable);
    }

    pub fn data_string(&self) -> &String {
        data!(self, String);
    }

    pub fn data_string_mut(&mut self) -> &mut String {
        data_mut!(self, String);
    }
}
