// Refactor `fastestCar` using `compose()` and other functions in pointfree-style.

// fastestCar :: [Car] -> String
const addIsFastest = (x) => concat(x, " is the fastest");
const fastestCar = compose(
  append(" is the fastest"),
  prop("name"),
  last,
  sortBy(prop("horsepower"))
);

// const fastestCar = (cars) => {
//   const sorted = sortBy(car => car.horsepower, cars);
//   const fastest = last(sorted);
//   return concat(fastest.name, ' is the fastest');
// };
