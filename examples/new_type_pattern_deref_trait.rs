/*!
mongodb 曾有用户反馈 Bson::DateTime 不好用，不符合人体工学(例如椅子坐起来不舒服)

<https://github.com/mongodb/bson-rust/issues/256>

结果 mongodb 官方人员看了后把 bson::DateTime(chrono::DateTime) 改成 i64 类型

明明 issue 的人都提到 Deref，气的 leader 大骂 mongodb 的人是不懂 Rust 的精髓 Deref 可以让类型包一层的包装对调用者透明么

Deref 的作用:
1. `A::len()` 等于调用 `A::Target::len()`
2. `*a` 等于访问 `a.0`
3. 自动解引用

如果说 new type pattern 是为了解决 trait orphan rule(什么是 orphan rule 可以看《Rust 编程之道》)

Deref/DerefMut 怎么解决 new type pattern 在使用上的不方便的?

对调用者来说 `A(Vec<i32>)` 的 `A()` 这层为了应对孤儿规则的 new type 包装是透明的

也就是调用者操作 A 跟 A 操作内部的 Vec<i32> 的代码完全一样

就好像这层包装和抽象完全不存在，使得 new type 对调用方来说是【透明的】，使得调用方操作 A 就像操作 Vec<i32> 一样方便
*/
struct A(Vec<i32>);

impl std::ops::Deref for A {
    type Target = Vec<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let a = A(vec![]);
    dbg!(a.len());
}
