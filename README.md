# Vigiland

Inhibit idle behaviour of a Wayland compositor.

## Installing

### Arch (AUR)

Thanks @kulothunganug for [packaging Vigiland on the AUR](https://aur.archlinux.org/packages/vigiland-git). You can install it using an AUR helper like yay or paru:

```
paru -S vigiland-git
```

### Nix

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

You can also run it in the background & use `killall vigiland` to stop inhibiting idle behaviour:

```bash
vigiland & disown
```

## Technical

Your compositor should support the `idle-inhibit-unstable-v1` protocol in order for Vigiland to work.
