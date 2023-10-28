# mpl3115a2-hal

Device driver for the MPL3115A2 pressure sensor using the embedded-hal traits.

Enable either async or blocking support with the `async` or `blocking` features respectively.
You cannot use both at the same time, due to the way they're implemented using the same functions.
