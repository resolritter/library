# Before building for production

Generate the environment for building the assets

```sh
node ./scripts/assemble_production_build_env.js
```

# Development setup

- `npm install`
- `npm start`

# Feature outline

- Renderer: `react`
- Router: `connected-react-router`
- State management: `react-redux`
- CSS: `linaria`
- Utility library: `lodash`

# Supporting packages

- `react-id-generator` generates unique IDs which can be scoped per component
  lifecycle (`useId`) or just generated whenever (`nextId`)
- `@reduxjs/toolkit` allows for creating reducers with less boilerplate. It's
  also useful outside of Redux (see `LocalIncrementButton.js`).
- `@svgr/webpack` allows importing SVGs as React components.
- `useForceUpdate` for manual updates. In the past, a combination of
  [useCounter](https://github.com/streamich/react-use/blob/master/src/useCounter.ts) +
  React's built-in `useReducer` was used for this purpose. One problem is that
  react-use does way more than this one use-case we are interested in, so it
  felt like overkill bringing the whole library for just that. Other minor
  problems led to `useForceUpdate` being chosen over it:
    - `useCounter` is less performant.
    - `useCounter` relies on `MAX_INT`, which might blow up at some point due
    to integer size limitations.
