<script lang="ts">
  import { canLogIn } from "~/lib/api/internal/token-store";
  import NavBar from "../../components/NavBar.svelte";
</script>

<NavBar title="">
  <span>
    <a class="btn {canLogIn ? 'btn-primary' : 'btn-ghost'}" href="/login"
      >Log In</a
    >
    <a class="btn {canLogIn ? 'btn-ghost' : 'btn-primary'}" href="/signup"
      >Sign Up</a
    >
  </span>
</NavBar>
<main>
  <article>
    <h1>Just A Chat App</h1>
  </article>
</main>

<style>
  main {
    flex: 1 1 auto;
  }
  a {
    margin: 16px;
  }
  h1 {
    display: flex;
    flex-basis: 100%;
    font-size: 64px;
    justify-content: center;
    text-align: center;
  }
</style>
