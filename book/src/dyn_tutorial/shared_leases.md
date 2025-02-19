# Shared leases

{{#include ../caveat.md}}

Earlier, we saw how you can [use the `share` keyword to create joint ownership](./share.md). You can also apply `share` to a **leased object**. In that case, the lease becomes a *shared lease*. Like any shared permission, it can be copied freely, and it doesn't permit mutation. Like a lease, it can be cancelled by the lessor. Unlike an exclusive lease, however, lessors cancel a shared lease by *writing*. They are permitted to read because shared leases allow anyone to read.

Shared leases are illustrated in the following program:

```
class Point(var x, var y)

async fn main() {
    var p = Point(x: 22, y: 44)
    var q = p.lease.share
    var r = q.share
    print("p is ({p.x}, {p.y})").await
    print("q is ({q.x}, {q.y})").await
    print("r is ({r.x}, {r.y})").await
    p.x += 1
}
```

Let's take it step by step. First, position your cursor after `p.lease` and you will see:

```
┌───┐
│   │                  ┌───────┐
│ p ├─my──────────────►│ Point │
│   │                  │ ───── │
│ q ├─leased(p)───────►│ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

Just after the `lease`, we have that `q` is leased from `p`, the owner. Move the cursor after the `.share` and we see that the exclusive lease is now a shared lease, indicated by `shared(p)`:

```
┌───┐
│   │                  ┌───────┐
│ p ├─my──────────────►│ Point │
│   │                  │ ───── │
│ q ├─shared(p)───────►│ x: 22 │
│   │                  │ y: 44 │
└───┘                  └───────┘
```

Although the lease is now shared, `p` remains the owner of the point (and the lessor of the lease).

Next go past the `var r = q.share` line. As always, sharing a shared thing simply reproduces it:

```
┌───┐
│   │                  
│ p ├─my──────────────►┌───────┐
│   │                  │ Point │
│ q ├─shared(p)───────►│ ───── │
│   │                  │ x: 22 │
│ r ├─shared(p)───────►│ y: 44 │
│   │                  └───────┘
└───┘                  
```

Finally, if you move your cursor to after `p.x += 1` you will see that the lease has expired, so `q` and `r` have no value:

```
┌───┐
│   │                  ┌───────┐
│ p ├─my──────────────►│ Point │
│   │                  │ ───── │
│ q │                  │ x: 23 │
│   │                  │ y: 44 │
│ r │                  └───────┘
│   │                  
└───┘                  
```

## Leasing a shared value

If `x.lease.share` produces a shared lease, what do you think happens with `x.share.lease`? In other words, what happens if we try to lease a shared value? The answer is: that is the same as sharing it. In other words, you get an equal copy to the original. In other words, `p`, `q`, `r`, and `s` here are all shared owners of the same `Point`:

```
class Point(var x, var y)

async fn main() {
    var p = Point(x: 22, y: 44).share
    var q = q.lease
    var r = q.share
    var s = q
}
```

Why is this? Well, there is nothing that `q` can do that would invalidate `p` (same with `r` and `s`). So there is no reason to create a lease. In other words, once you've started sharing, you've already created a permission that can be duplicated at will, so there is no reason not to keep duplicating it further.