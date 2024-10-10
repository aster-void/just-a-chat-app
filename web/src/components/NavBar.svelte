<header>
  <div class="navbar bg-base-60 border-10 border-base-500">
    <div class="navbar-start">
      <a href="/" class="btn btn-ghost text-xl">daisyUI</a>
    </div>
    <div class="navbar-end">
      <slot />
    </div>
  </div>
</header>

<style>
  header {
    position: sticky;
    height: 10vh;
    top: 0;
    backdrop-filter: blur(30px);
  }
</style>
