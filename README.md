# Vigiland

Inhibit idle behaviour of a Wayland compositor.

## Installing

Add the flake as an input:

```nix
vigiland.url = "github:jappie3/vigiland";
```

Install the package:

```nix
environment.systemPackages = [inputs.vigiland.packages.${pkgs.system}.vigiland];
```

## Usage

Run it, ctrl+c to exit:

```bash
vigiland
```
