
<div align="center" style="padding-top:10rem">

<img src="./rsc/shine_banner_dark.png" height="130em">

<i><b>SH</b>ell <b>IN</b>terface <b>E</b>ngine</i>    

<p>A Shell-based GUI Engine to build complex and reactive desktop apps directly from your shell</p>

</div>


## Create Desktop Apps From Shell Only

Every programming language has its dedicated GUI toolkit: Python has Tkinter, Go has Fyne, Rust has iced. But what about the Shell?

**Shine** is the missing link. It is a lightweight desktop GUI engine built specifically to bring fully customizable, reactive interfaces directly to your shell scripts (supporting any shell language).

With *SXML* (Shine XML), you get complete control over styling, layouts, components, and interactive callbacks. Giving you a modern, web-like development experience right in your terminal workflow.



## Why Shine?

- ⚡ **Shell-First Power** : Leverage the full power of Bash, Zsh, or any scripting language.make it easier thant never to create desktop app taht can interact with your system

- 🔄 **Pure MVU Architecture** : Enjoy a predictable, uni-directional data flow with simple architecture. Don’t have to learn a complexe react like architecture juste for pop few button and text

- 🛠️ **Built for System Control** : Tailor-made for utilities, status bars extensions, custom launchers, and desktop ricing workflows.

- 🦀 **Powered by Rust** : Under the hood, Shine is fast, secure, and resource-efficient. No heavy browser runtimes, no massive memory footprints. Using iced as GUI engine

<br> <br> <br>

#### _The Backstory
_This project was born out of frustration.
Running NixOS with a minimal compositor (Niri), I had zero default widgets.
Doing basic stuff like shutting down or checking CPU temperature required typing commands, which is fun, but a nice widget-like app is pleasant too._

_So, I built Shine to unify the way we build tiny UIs with system interaction (or not). It's my attempt at solving this. It is what it is, and hopefully, it can help you too!_