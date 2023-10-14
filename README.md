This is just an experiment to play with RedPanda, Nuxt (and a bit of partyjs?), more rust and a bit of github actions.

The goal here is to wire up some sort of fish and chip shop that can take orders of passer-bys and serve them up.

Recipe Service
- Defines how to make a burger, fries and a drink
    - This includes how long to wait
- How much it would cost for now

Chef Service
- Does the actual cooking work
- Takes one portion of an order, asks the Recipe service how to assemble, processes or waits on the order, providing updates as parts are done, then responds with a completed order, sends updates to the queue, sends the order itself (with hash codes) when done to queue

Cashier Service
- Receives customers order, converts to order into chunk orders, posts chunks to queue for chef service
- Also gives updates on persons order - assume no id, just issues a random id for customer

A recipe is a combination of components
- For now keep it simple, a collection of ids, some are marked as parallel others serial, for now assume all parallel

Once working, look at getting some telemetry working