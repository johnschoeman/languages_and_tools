import { WebSocket, WebSocketServer } from "ws"
import { v4 as uuid } from 'uuid';

type Msg = Record<string, any>

type UUID = string

type Subscriber = {
  id: UUID,
  next: (msg: Msg) => void
}

type Subscription = () => void

const removeById = (id: string, subscribers: Subscriber[]): void => {

}

const subscribers: Subscriber[] = []

const subscribe = (
  listener: Subscriber,
): Subscription => {
  subscribers.push(listener)
  const id = uuid()

  return {
    id,
    remove: () => removeById(id, subscribers)
    next: (msg: Msg) => listener(msg)
  }
}

const publish = (msg: Record<string, any>) => {
  if (subscribers.length < 1) {
    console.log("No subscribers")
    return
  }

  subscribers.forEach(subscriber => {
    subscriber.next(msg)
  })
}

const wsHost = "127.0.0.1"
const wsPath = "/websocket"
const wsPort = 4000

const wsServer = new WebSocketServer({
  host: wsHost,
  port: wsPort,
  path: wsPath,
})

wsServer.on("listening", () => {
  console.log(`WebSServer running at ws://${wsHost}:${wsPort}`)
})

wsServer.on("connection", ws => {
  console.log("Connecting ws")

  const remove = subscribe(subscriber => {
    ws.send(JSON.stringify(json))
  })

  ws.on("message", (data) => {
    console.log('received: %s', data);
    subscribers.forEach(subscriber => {
      subscriber.next(data)
    })
  })

  ws.on("close", () => {
    console.log("Closing ws")
    remove()
  })
})


