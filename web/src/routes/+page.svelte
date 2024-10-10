<script lang="ts">
  import NavBar from "../components/NavBar.svelte";
</script>

<main>
  <NavBar>
    <button class="btn btn-ghost">Log In</button>
    <button class="btn btn-primary">Sign Up</button>
  </NavBar>
  <article>
    <h1>Just A Chat App</h1>
  </article>
</main>

<style>
  button {
    margin: 16px;
  }
  h1 {
    font-size: 64px;
    text-align: center;
  }
  main {
    min-height: 200vh;
  }
</style>
