# Polynomial

## Evaluate polynomial function

### Example

```console
rustamath-polynomial eval -v 1.2345 1.1 2.2 3.3 4.4
f(1.2345) = 17.12307806495
1.1*x^0 + 2.2*x^1 + 3.3*x^2 + 4.4*x^3
```

### Usage

```console
rustamath-polynomial eval --help
Evaluate polynomial f(x)

Usage: rustamath-polynomial eval [OPTIONS] <X> <COEFFS>...

Arguments:
  <X>          x in f(x)
  <COEFFS>...  Coefficients c0, c1, c2...

Options:
  -v, --verbose  Verbose output
  -h, --help     Print help
```

Number of coefficients define degree of polynomial function.
The order of coefficients:

```console
c0 + c1*x + c2*x^2 + ... + ci*x^i
```

## Plot polynomial function

### Example

```console
rustamath-polynomial plot -f $OUT/poly1 -s=-10 -e 10 -d 8 -- 200 -5 -1 0.1
```

![Plot](../image/poly1.svg)