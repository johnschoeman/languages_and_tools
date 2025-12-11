import {
  Subject,
  merge,
  of,
  timer,
  EMPTY,
  fromEvent,
  interval,
  asyncScheduler,
  combineLatest,
  BehaviorSubject,
} from "rxjs";
import * as RX from "rxjs/operators";

const observer = {
  next: val => console.log("next", val),
  error: err => console.log("error", err),
  complete: () => console.log("complete")
}

// const interval$ = interval(500).pipe(
//   RX.tap(i => console.log("interval", i))
// )
// const multicastInterval$ = interval$.pipe(
//   RX.share()
// )
// const subOne = multicastInterval$.subscribe(observer)
// const subTwo = multicastInterval$.subscribe(observer)
// setTimeout(() => {
//   subTwo.unsubscribe()
//   subOne.unsubscribe()
// }, 3000)

// const subject = new BehaviorSubject("HEEEE")
//
// const subscription = subject.subscribe(
//   observer
// )
//
// subject.next("HEMML")
//
// const subscription2 = subject.subscribe(observer)
//
// subject.next("WOFL")
//
// setTimeout(() => {
//   console.log(subject.getValue())
// }, 3000)
