use type_change::From;

#[derive(Debug, PartialEq, Eq, Clone)]
struct OldState {
    id: i64,
    name: String,
}

#[derive(Debug, PartialEq, Eq, From)]
#[from(OldState)]
struct NewState {
    id: i64,
    name: String,
}

#[test]
fn named_struct() {
    let old = OldState {
        id: 1,
        name: "Panda".to_string(),
    };
    let new: NewState = old.clone().into();
    assert_eq!(
        new,
        NewState {
            id: old.id,
            name: old.name
        }
    );
}

#[derive(Debug, PartialEq, Eq, From)]
struct NewType(OldState);

#[test]
fn unnamed_struct() {
    let old = OldState {
        id: 1,
        name: "Panda".to_string(),
    };
    let newtype: NewType = old.clone().into();
    assert_eq!(newtype, NewType(old));
}

mod nested {
    use super::*;

    mod new {
        use super::*;

        #[derive(Debug, PartialEq, Eq, Clone, From)]
        #[from(old::Parent)]
        pub struct Parent {
            pub id: i64,
            pub child: Child,
        }

        #[derive(Debug, PartialEq, Eq, Clone, From)]
        #[from(old::Child)]
        pub struct Child {
            pub value: String,
        }
    }

    mod old {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct Parent {
            pub id: i64,
            pub child: Child,
        }

        #[derive(Debug, PartialEq, Eq, Clone)]
        pub struct Child {
            pub value: String,
        }
    }

    #[test]
    fn named_struct() {
        let old = old::Parent {
            id: 1,
            child: old::Child {
                value: "Panda".to_string(),
            },
        };
        let new: new::Parent = old.clone().into();
        assert_eq!(
            new,
            new::Parent {
                id: old.id,
                child: new::Child {
                    value: old.child.value
                }
            }
        );
    }

    #[derive(Debug, PartialEq, Eq, From)]
    struct NewType(old::Parent);

    #[test]
    fn unnamed_struct() {
        let old = old::Parent {
            id: 1,
            child: old::Child {
                value: "Panda".to_string(),
            },
        };
        let newtype: NewType = old.clone().into();
        assert_eq!(newtype, NewType(old));
    }
}
