impl <A,D> MyTrait<A,D> for T
where
    A:MyTrait<A,D>,
    D:MyTrait<A,D>{}