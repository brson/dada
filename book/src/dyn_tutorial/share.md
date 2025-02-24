# Sharing and shared leases

{{#include ../caveat.md}}

We are now coming to the end of our tour of Dada's permissions. In the previous few chapters we saw:

* When you first create an object, you are its [owner](./create.md). When the owning variable goes out of scope, the object is freed.
* Owners can [give](./give.md) their permissions to others, but then the owner no longer has permission.
* Owners can [lease](./lease.md) their permission, giving other variables exclusive access to the object until the owner uses it again.

You may have noticed a theme here: access to the object is "linear", it moves along a line from variable to variable. There is no point where two variables can be used to access the value. It might seem like leases are an exception, but that's not quite true: with an (exclusive) lease, first the lessee gets access, and then later the lessor reclaims access, but at no point do both of them have access.

## The share keyword

The `share` keyword is used to convert an *unique* permission into a *shared* permission. Unlike unique permissions, shared permissions can be copied freely into many variables, and all of them are considered equivalent. Let's start with shared ownership, and later we'll talk about sharing a leased value.

Consider this program:

```
class Point(var x, var y)

async fn main() {
    var p = Point(x: 22, y: 44)
    var q = p.share
    print("The point is ({p.x}, {p.y})").await
    print("The point is ({q.x}, {q.y})").await
}
```

The expression `p.share` means that `p` is converting its unique ownership into shared ownership; `p` and `q` are now joint owners of the same `Point`. If we move the cursor to just after that line we will [see](https://asciiflow.com/#/share/eJyrVspLzE1VssorzcnRUcpJrEwtUrJSqo5RqohRsrK0MNOJUaoEsowsDYGsktSKEiAnRunRlD3IKCYmD0gqKChASDSAphiLxgKomtxKPGrR0bRdIK0B%2BZl5JQrEuAHdHUiaCvG5kbAbKqwUjIyIc0OllYKJCUIpintICjilWqVaANIL5SU%3D) that both of them have the `our` permission:

```
┌───┐
│   │                  ┌───────┐
│ p ├─our─────────────►│ Point │
│   │                  │ ───── │
│ q ├─our─────────────►│ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

## Joint ownership

Objects with multiple owners are freed once *all* of their owners have gone out of scope. Let's explore this with this example:

```
async fn main() {
    var p = Point(x: 22, y: 44)
    print("The point is ({p.x}, {p.y})").await
    var q = p.share
    print("The point q is ({q.x}, {q.y})).await
}
```

Position the cursor right before `var q = p.share`. You will see:

```
┌───┐
│   │                  ┌───────┐
│ p ├─my──────────────►│ Point │
│   │                  │ ───── │
│ q │                  │ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

Now move the cursor right *after* `q = p.share`. You will see:

```
┌───┐
│   │                  ┌───────┐
│ p ├─our─────────────►│ Point │
│   │                  │ ───── │
│ q ├─our─────────────►│ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

The `my` permission from `p` has been converted to an `our` permission, and `q` also has `our` permission. OK, let's move one step forward, to right before the `print`. Now we see:

```
┌───┐
│   │                  ┌───────┐
│ p │                  │ Point │
│   │                  │ ───── │
│ q ├─our─────────────►│ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

`p` is no longer in active use, so the value for `p` has been dropped, but the `Point` is not freed. That's because it still has one owner (`q`). 

Notice that `q` still only has `our` permission, not `my`. Once an object is shared, it remains shared. This is because `q` doesn't know how many other variables there are that may have access to the `Point`, so it always acts "as if" there are more. There are ways to test at runtime whether you are the only owner left and convert an `our` permission back into a `my` permission, but we'll discuss that later.

If you like, step forward a few more steps in the debugger: you'll see that once `q` goes out of scope, the `Point` is dropped completely, since it no longer has any owners.

## Sharing and giving a shared thing

Once something is shared, we can go on and share it even further:

```
class Point(var x, var y)

async fn main() {
    var p = Point(x: 22, y: 44)
    var q = p.share
    var r = q.share
    var s = r.share
    // ...and so on
}
```

Each time we share a jointly owned object like the `Point` here, we just add one more owner.

Similarly, since all shared variables are equal, when a shared variable gives its permissions to another, that is equivalent to sharing again. In the following program, `p`, `q`, `r`, and `s` are all joint owners of the same `Point`:

```
class Point(var x, var y)

async fn main() {
    var p = Point(x: 22, y: 44)
    var q = p.share
    var r = q.give
    var s = r        // equivalent to r.give
}
```
