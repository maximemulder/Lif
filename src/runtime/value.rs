use crate::runtime::data::{ Array, Class, Data, Function, Generic, Method, Nullable, Object };
use crate::runtime::engine::Engine;
use crate::runtime::error::Error;
use crate::runtime::gc::{ GcRef, GcTrace };
use crate::runtime::r#return::{ Return, ReturnReference, ReturnValue };
use crate::runtime::utilities::parameters;
use crate::runtime::utilities::tag::Tag;

pub type GcValue<'a> = GcRef<Value<'a>>;

pub struct Value<'a> {
    pub class: GcValue<'a>,
    data: Data<'a>,
}

pub trait Primitive<'a> {
    fn get(engine: &Engine<'a>, value: GcValue<'a>) -> Self;
}

pub trait PrimitivePtr<'a> {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self;
    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self;
}

impl<'a> Primitive<'a> for bool {
    fn get(engine: &Engine<'a>, value: GcValue<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.boolean);
        if let Data::Boolean(boolean) = value.data {
            return boolean;
        }

        panic!();
    }
}

impl<'a> Primitive<'a> for isize {
    fn get(engine: &Engine<'a>, value: GcValue<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.integer);
        if let Data::Integer(integer) = value.data {
            return integer;
        }

        panic!();
    }
}

impl<'a> Primitive<'a> for f64 {
    fn get(engine: &Engine<'a>, value: GcValue<'a>) -> Self {
        debug_assert!(value.class == engine.primitives.float);
        if let Data::Float(float) = value.data {
            return float;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Array<'a> {
    fn get_ref<'b>(_: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        // TODO assert array class
        if let Data::Array(array) = value.data() {
            return array;
        }

        panic!();
    }

    fn get_mut<'b>(_: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        // TODO assert array class
        if let Data::Array(array) = value.data_mut() {
            return array;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Class<'a> {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        debug_assert!(value.class == engine.primitives.class);
        if let Data::Class(class) = value.data() {
            return class;
        }

        panic!();
    }

    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        debug_assert!(value.class == engine.primitives.class);
        if let Data::Class(class) = value.data_mut() {
            return class;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Function<'a> {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        debug_assert!(value.class == engine.primitives.function);
        if let Data::Function(function) = value.data() {
            return function;
        }

        panic!();
    }

    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        debug_assert!(value.class == engine.primitives.function);
        if let Data::Function(function) = value.data_mut() {
            return function;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Generic<'a> {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        debug_assert!(value.class == engine.primitives.generic);
        if let Data::Generic(generic) = value.data() {
            return generic;
        }

        panic!();
    }

    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        debug_assert!(value.class == engine.primitives.generic);
        if let Data::Generic(generic) = value.data_mut() {
            return generic;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Method<'a> {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        debug_assert!(value.class == engine.primitives.method);
        if let Data::Method(method) = value.data() {
            return method;
        }

        panic!();
    }

    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        debug_assert!(value.class == engine.primitives.method);
        if let Data::Method(method) = value.data_mut() {
            return method;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Nullable<'a> {
    fn get_ref<'b>(_: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        // TODO assert nullable class
        if let Data::Nullable(nullable) = value.data() {
            return nullable;
        }

        panic!();
    }

    fn get_mut<'b>(_: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        // TODO assert nullable class
        if let Data::Nullable(nullable) = value.data_mut() {
            return nullable;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for Object<'a> {
    fn get_ref<'b>(_: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        // TODO assert object class
        if let Data::Object(object) = value.data() {
            return object;
        }

        panic!();
    }

    fn get_mut<'b>(_: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        // TODO assert object class
        if let Data::Object(object) = value.data_mut() {
            return object;
        }

        panic!();
    }
}

impl<'a> PrimitivePtr<'a> for String {
    fn get_ref<'b>(engine: &Engine<'a>, value: &'b GcValue<'a>) -> &'b Self {
        debug_assert!(value.class == engine.primitives.string);
        if let Data::String(string) = value.data() {
            return string;
        }

        panic!();
    }

    fn get_mut<'b>(engine: &Engine<'a>, value: &'b mut GcValue<'a>) -> &'b mut Self {
        debug_assert!(value.class == engine.primitives.string);
        if let Data::String(string) = value.data_mut() {
            return string;
        }

        panic!();
    }
}

impl <'a> GcValue<'a> {
    pub fn get<T: Primitive<'a>>(self, engine: &Engine<'a>) -> T {
        T::get(engine, self)
    }

    pub fn get_ref<T: PrimitivePtr<'a>>(&self, engine: &Engine<'a>) -> &T {
        T::get_ref(engine, self)
    }

    pub fn get_mut<T: PrimitivePtr<'a>>(&mut self, engine: &Engine<'a>) -> &mut T {
        T::get_mut(engine, self)
    }
}

impl<'a> Value<'a> {
    pub fn new(class: GcValue<'a>, data: Data<'a>) -> Self {
        Self {
            class,
            data,
        }
    }

    pub fn data(&self) -> &Data<'a> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Data<'a> {
        &mut self.data
    }
}

impl<'a> GcValue<'a> {
    pub fn is(self, engine: &Engine<'a>, class: GcValue<'a>) -> bool {
        if self == class {
            true
        } else if let Some(parent) = self.get_ref::<Class>(engine).parent() {
            parent.is(engine, class)
        } else {
            false
        }
    }

    pub fn is_generic(self, engine: &Engine<'a>, generic: GcValue<'a>) -> bool {
        if let Some(constructor) = self.get_ref::<Class>(engine).constructor() {
            if constructor.generic == generic {
                return true;
            }
        }

        false
    }
}

impl<'a> GcValue<'a> {
    pub fn isa(self, engine: &Engine<'a>, class: GcValue<'a>) -> bool {
        self.class.is(engine, class)
    }

    pub fn isa_generic(self, engine: &Engine<'a>, generic: GcValue<'a>) -> bool {
        self.class.is_generic(engine, generic)
    }

    pub fn cast(self, engine: &Engine<'a>, class: GcValue<'a>) -> Return<()> {
        if self.isa(engine, class) {
            Ok(())
        } else {
            Err(error_cast(engine, self, class))
        }
    }
}

impl<'a> GcValue<'a> {
    pub fn get_method(&self, engine: &Engine<'a>, name: &str) -> ReturnValue<'a> {
        if let Some(method) = self.class.get_ref::<Class>(engine).get_method(engine, name) {
            Ok(method)
        } else {
            Err(error_undefined_method(engine, name, self.class))
        }
    }

    pub fn call_method(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        let mut values = Vec::new();
        values.push(self);
        values.extend_from_slice(arguments);
        self.call_method_self(engine, name, &mut values)
    }

    pub fn call_method_self(self, engine: &mut Engine<'a>, name: &str, arguments: &mut [GcValue<'a>]) -> ReturnReference<'a> {
        let method = self.get_method(engine, name)?;
        let array = parameters::pack(engine, arguments);
        method.get_method(engine, "__cl__")?.get_ref::<Function>(engine).call(engine, &mut [method, array])
    }

    pub fn call_fstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__fstr__", &mut [])?.read()?.get_ref::<String>(engine).clone())
    }

    pub fn call_sstr(self, engine: &mut Engine<'a>) -> Return<String> {
        Ok(self.call_method(engine, "__sstr__", &mut [])?.read()?.get_ref::<String>(engine).clone())
    }
}

impl<'a> GcValue<'a> {
    pub fn get_cast_array(&self, engine: &Engine<'a>) -> Return<&Array<'a>> {
        if self.isa_generic(engine, engine.primitives.array) {
            Ok(self.get_ref(engine))
        } else {
            Err(error_cast(engine, *self, engine.primitives.array_any))
        }
    }

    pub fn get_cast_boolean(self, engine: &Engine<'a>) -> Return<bool> {
        self.cast(engine, engine.primitives.boolean)?;
        Ok(self.get(engine))
    }
}

impl GcTrace for Value<'_> {
    fn trace(&mut self) {
        self.class.trace();
        self.data.trace();
    }
}

impl<'a> Value<'a> {
    pub fn data_tag(&self) -> Tag {
        match &self.data {
            Data::Class(class)       => class.tag().clone(),
            Data::Function(function) => function.tag().clone(),
            Data::Generic(generic)   => generic.tag().clone(),
            _ => panic!(),
        }
    }
}

fn error_undefined_method<'a>(engine: &Engine<'a>, method: &str, class: GcValue<'a>) -> Error {
    Error::new_runtime(&format!("Method `{}` is undefined for type `{}`.", method, class.get_ref::<Class>(engine).tag()))
}

fn error_cast<'a>(engine: &Engine<'a>, value: GcValue<'a>, r#type: GcValue<'a>) -> Error {
    Error::new_runtime(&format!("Cannot cast a value of the type `{}` to the type `{}`.", value.class.get_ref::<Class>(engine).tag(), r#type.get_ref::<Class>(engine).tag()))
}
