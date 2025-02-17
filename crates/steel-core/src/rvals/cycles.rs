use super::*;

#[derive(Default)]
// Keep track of any reference counted values that are visited, in a pointer
pub(super) struct CycleDetector {
    // Keep a mapping of the pointer -> gensym
    visited: std::collections::HashSet<usize>,

    // Recording things that have already been seen
    cycles: std::collections::HashMap<usize, usize>,

    // Values captured in cycles
    values: Vec<SteelVal>,
}

impl CycleDetector {
    pub(super) fn detect_and_display_cycles(val: &SteelVal, f: &mut fmt::Formatter) -> fmt::Result {
        let mut detector = CycleDetector::default();

        detector.visit(val);

        detector.start_format(val, f)
    }

    fn start_format(self, val: &SteelVal, f: &mut fmt::Formatter) -> fmt::Result {
        // todo!()

        for node in &self.values {
            let id = match &node {
                SteelVal::CustomStruct(c) => {
                    let ptr_addr = c.as_ptr() as usize;
                    self.cycles.get(&ptr_addr).unwrap()
                }
                _ => {
                    unreachable!()
                }
            };

            write!(f, "#{id}=")?;
            self.top_level_format_with_cycles(node, f)?;
            writeln!(f)?;
        }

        if !self.values.contains(val) {
            self.format_with_cycles(val, f)?;
        }

        Ok(())

        // for node in
    }

    fn top_level_format_with_cycles(&self, val: &SteelVal, f: &mut fmt::Formatter) -> fmt::Result {
        match val {
            BoolV(b) => write!(f, "#{b}"),
            NumV(x) => write!(f, "{x:?}"),
            IntV(x) => write!(f, "{x}"),
            StringV(s) => write!(f, "{s:?}"),
            CharV(c) => write!(f, "#\\{c}"),
            FuncV(_) => write!(f, "#<function>"),
            Void => write!(f, "#<void>"),
            SymbolV(s) => write!(f, "{s}"),
            VectorV(lst) => {
                let mut iter = lst.iter();
                write!(f, "(")?;
                if let Some(last) = iter.next_back() {
                    for item in iter {
                        self.format_with_cycles(item, f)?;
                        write!(f, " ")?;
                    }
                    self.format_with_cycles(last, f)?;
                }
                write!(f, ")")
            }
            Custom(x) => write!(f, "#<{}>", x.borrow().display()?),
            CustomStruct(s) => {
                // if let Some(id) = self.cycles.get(&(s.as_ptr() as usize)) {
                //     write!(f, "#{}#", id)
                // } else {
                let guard = s.borrow();

                {
                    if s.borrow()
                        .properties
                        .get(&SteelVal::SymbolV(SteelString::from("#:transparent")))
                        .is_some()
                    {
                        write!(f, "({}", guard.name)?;
                        for i in 0..guard.len - 1 {
                            write!(f, " ")?;
                            self.format_with_cycles(&guard.fields[i], f)?;
                        }

                        write!(f, ")")
                    } else {
                        write!(f, "({})", guard.name)
                    }
                }
                // }
            }

            PortV(_) => write!(f, "#<port>"),
            Closure(_) => write!(f, "#<bytecode-closure>"),
            HashMapV(hm) => write!(f, "#<hashmap {:#?}>", hm.as_ref()),
            IterV(_) => write!(f, "#<iterator>"),
            HashSetV(hs) => write!(f, "#<hashset {hs:?}>"),
            FutureFunc(_) => write!(f, "#<future-func>"),
            FutureV(_) => write!(f, "#<future>"),
            // Promise(_) => write!(f, "#<promise>"),
            StreamV(_) => write!(f, "#<stream>"),
            Contract(c) => write!(f, "{}", **c),
            ContractedFunction(_) => write!(f, "#<contracted-function>"),
            BoxedFunction(_) => write!(f, "#<function>"),
            ContinuationFunction(c) => write!(f, "#<continuation: {:?}>", c.stack),
            #[cfg(feature = "jit")]
            CompiledFunction(_) => write!(f, "#<compiled-function>"),
            ListV(l) => {
                write!(f, "(")?;

                let mut iter = l.iter().peekable();

                while let Some(item) = iter.next() {
                    self.format_with_cycles(item, f)?;
                    if iter.peek().is_some() {
                        write!(f, " ")?
                    }
                }

                // for item in l.iter().pe

                // for item in l {
                //     display_helper(item, f)?;
                //     write!(f, " ")?;
                // }
                write!(f, ")")
            }
            // write!(f, "#<list {:?}>", l),
            MutFunc(_) => write!(f, "#<function>"),
            BuiltIn(_) => write!(f, "#<function>"),
            ReducerV(_) => write!(f, "#<reducer>"),
            MutableVector(v) => write!(f, "{:?}", v.as_ref().borrow()),
            SyntaxObject(s) => write!(f, "#<syntax:{:?}:{:?} {:?}>", s.source, s.span, s.syntax),
            BoxedIterator(_) => write!(f, "#<iterator>"),
            Boxed(b) => write!(f, "'#&{}", b.get()),
        }
    }

    fn format_with_cycles(&self, val: &SteelVal, f: &mut fmt::Formatter) -> fmt::Result {
        match val {
            BoolV(b) => write!(f, "#{b}"),
            NumV(x) => write!(f, "{x:?}"),
            IntV(x) => write!(f, "{x}"),
            StringV(s) => write!(f, "{s:?}"),
            CharV(c) => write!(f, "#\\{c}"),
            FuncV(_) => write!(f, "#<function>"),
            Void => write!(f, "#<void>"),
            SymbolV(s) => write!(f, "{s}"),
            VectorV(lst) => {
                let mut iter = lst.iter();
                write!(f, "(")?;
                if let Some(last) = iter.next_back() {
                    for item in iter {
                        self.format_with_cycles(item, f)?;
                        write!(f, " ")?;
                    }
                    self.format_with_cycles(last, f)?;
                }
                write!(f, ")")
            }
            Custom(x) => write!(f, "{}", x.borrow().display()?),
            CustomStruct(s) => {
                if let Some(id) = self.cycles.get(&(s.as_ptr() as usize)) {
                    write!(f, "#{id}#")
                } else {
                    let guard = s.borrow();

                    {
                        if s.borrow()
                            .properties
                            .get(&SteelVal::SymbolV(SteelString::from("#:transparent")))
                            .is_some()
                        {
                            write!(f, "({}", guard.name)?;
                            for i in 0..guard.len - 1 {
                                write!(f, " ")?;
                                self.format_with_cycles(&guard.fields[i], f)?;
                            }

                            write!(f, ")")
                        } else {
                            write!(f, "({})", guard.name)
                        }
                    }
                }
            }

            PortV(_) => write!(f, "#<port>"),
            Closure(_) => write!(f, "#<bytecode-closure>"),
            HashMapV(hm) => write!(f, "#<hashmap {:#?}>", hm.as_ref()),
            IterV(_) => write!(f, "#<iterator>"),
            HashSetV(hs) => write!(f, "#<hashset {hs:?}>"),
            FutureFunc(_) => write!(f, "#<future-func>"),
            FutureV(_) => write!(f, "#<future>"),
            // Promise(_) => write!(f, "#<promise>"),
            StreamV(_) => write!(f, "#<stream>"),
            Contract(c) => write!(f, "{}", **c),
            ContractedFunction(_) => write!(f, "#<contracted-function>"),
            BoxedFunction(_) => write!(f, "#<function>"),
            ContinuationFunction(c) => write!(f, "#<continuation: {:?}>", c.stack),
            #[cfg(feature = "jit")]
            CompiledFunction(_) => write!(f, "#<compiled-function>"),
            ListV(l) => {
                write!(f, "(")?;

                let mut iter = l.iter().peekable();

                while let Some(item) = iter.next() {
                    self.format_with_cycles(item, f)?;
                    if iter.peek().is_some() {
                        write!(f, " ")?
                    }
                }

                // for item in l.iter().pe

                // for item in l {
                //     display_helper(item, f)?;
                //     write!(f, " ")?;
                // }
                write!(f, ")")
            }
            // write!(f, "#<list {:?}>", l),
            MutFunc(_) => write!(f, "#<function>"),
            BuiltIn(_) => write!(f, "#<function>"),
            ReducerV(_) => write!(f, "#<reducer>"),
            MutableVector(v) => write!(f, "{:?}", v.as_ref().borrow()),
            SyntaxObject(s) => write!(f, "#<syntax:{:?}:{:?} {:?}>", s.source, s.span, s.syntax),
            BoxedIterator(_) => write!(f, "#<iterator>"),
            Boxed(b) => write!(f, "'#&{}", b.get()),
        }
    }

    fn add(&mut self, val: usize, steelval: &SteelVal) -> bool {
        if self.visited.contains(&val) {
            let id = self.cycles.len();

            // If we've already seen this, its fine, we can just move on
            if let std::collections::hash_map::Entry::Vacant(e) = self.cycles.entry(val) {
                e.insert(id);
                // Keep track of the actual values that are being captured
                self.values.push(steelval.clone());
            } else {
                return true;
            }

            return true;
        }

        self.visited.insert(val);
        false
    }

    // TODO: Complete the rest of this visitor
    fn visit(&mut self, val: &SteelVal) {
        match val {
            SteelVal::CustomStruct(s) => {
                // todo!()

                //
                if !self.add(s.as_ptr() as usize, val) {
                    for val in &s.borrow().fields {
                        self.visit(val);
                    }
                }
            }
            SteelVal::ListV(l) => {
                for val in l {
                    self.visit(val);
                }
            }
            SteelVal::HashMapV(h) => {
                for (key, val) in h.iter() {
                    self.visit(key);
                    self.visit(val);
                }
            }
            _ => {}
        }
    }
}
