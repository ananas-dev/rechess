<script>
  import { onDestroy } from "svelte";
  import { user } from "../stores/signup.js";

  const SERVER_URL = "http://localhost:3000";

  let user_value;

  const unsubscribe = user.subscribe(value => {
    user_value = value;
  });

  const validate = () => {
    console.log(user_value);

    fetch(`${SERVER_URL}/signup`, {
      method: "POST",
      body: user_value
    }).then(
      resp => resp.json().then(body => {
        console.log("Got response: " + body)
      })
    )
  }

  onDestroy(unsubscribe)
</script>

<div class=form-container>
  <h1>Register</h1>
  <form class=form-content on:submit|preventDefault={validate}>
    <label for=username>Username</label>
    <input class=username type=text bind:value={$user.name} />
    <label for=email>Email</label>
    <input class=email type=text bind:value={$user.email} />
    <label for=password>Password</label>
    <input class=password type=text bind:value={$user.password} />
    <br/>
    <button type=submit>Sign Up</button>
  </form>
</div>

<style>
  .form-container {
    background: #fff;
    border-radius: 2px;
    display: inline-block;
    height: 500px;
    width: 500px;
    margin: 1rem;
    position: relative;
  }

  /*
  .form-content {
    display: grid;
    grid-template-columns: 20% 80%;
    column-gap: 10px;
  }
  */
</style>
