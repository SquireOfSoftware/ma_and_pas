This is just an experiment to play with RedPanda.

The goal here is to wire up some sort of fish and chip shop that can take orders of passer-bys and serve them up.

The architecture will take this form:

![](architecture.png)

Where we will have:
- a trucker app that publishes and receives the order
- a shop app that will receive and deliver the order

The architecture may grow from here, but the important thing here is the scale at which this will operate on.

To test the consumption and sourcing of events in RedPanda and what happens if you take a particular app down.