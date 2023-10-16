This is just an experiment to play with RedPanda, Nuxt (and a bit of partyjs?), more rust and a bit of github actions.

The goal here is to wire up some sort of fish and chip shop that can take orders of passer-bys and serve them up.

Waiter Service
- This is just picking up the acknowledgements from the chef service and reports back the meal

Chef Service
- Does the actual cooking work
- Takes one portion of an order, asks the Recipe service how to assemble, processes or waits on the order, providing updates as parts are done, then responds with a completed order, sends updates to the queue, sends the order itself (with hash codes) when done to queue

Cashier Service
- Receives customers order, converts to order into chunk orders, posts chunks to queue for chef service
- Also gives updates on persons order - assume no id, just issues a random id for customer

A recipe is a combination of components
- For now keep it simple, a collection of ids, some are marked as parallel others serial, for now assume all parallel

Once working, look at getting some telemetry working

Redpand notes:

https://docs.redpanda.com/current/get-started/quick-start/?tab=tabs-1-single-broker

The port that you want your producer client on is 19092.

To test consumption:
```
docker exec -it redpanda-0 rpk topic consume test --num 1
```

I used this site for the Kotlin setup: https://medium.com/@abhikulshrestha22/kafka-producer-and-consumer-using-spring-boot-in-kotlin-100ce2a52fbd
