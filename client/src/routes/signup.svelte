<script context="module">
  export const prerender = true;
</script>

<script>
  import { onDestroy, onMount } from "svelte";
  //import { user } from "../stores/signup.js";

  const SERVER_URL = import.meta.env.VITE_SERVER_URL;

  let user = {
    username: "",
    email: "",
    password: "",
  };

  let password_confirmation;

  let form_error = "";

  /*
  const unsubscribe = user.subscribe((value) => {
    user_value = value;
  });
  */

  const validate = () => {
    if (user.password === password_confirmation) {
      fetch(`${SERVER_URL}/api/v1/users/`, {
        method: "POST",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify(user),
      }).then((resp) =>
        resp.text().then((body) => {
          console.log("Got response: " + body);
        })
      );
    } else {
      user.password = ""
      password_confirmation = ""
      form_error = "Passwords didn't match!"
    }
  };

  /*
  onDestroy(unsubscribe);
  */
</script>

<div class="content">
  <div class="signup-box">
    <h1>Register</h1>
    <form class="form-content" on:submit|preventDefault={validate}>
      <input
        class="username"
        placeholder="Username"
        type="text"
        bind:value={user.username}
      />
      <input
        class="email"
        placeholder="Email"
        type="text"
        bind:value={user.email}
      />
      <input
        class="password"
        placeholder="Password"
        type="password"
        bind:value={user.password}
      />
      <input
        class="password"
        placeholder="Confirm password"
        type="password"
        bind:value={password_confirmation}
      />
      <p class="form-error">
        {form_error}
      </p>
      <button type="submit" class="submit-btn">Sign Up</button>
    </form>
  </div>
</div>

<style>
  .content {
    display: flex;
    align-content: center;
    align-items: center;
    justify-content: center;
  }

  .signup-box {
    box-sizing: border-box;
    padding: 50px;
    background-color: #eee;
    border-radius: 20px;
    box-shadow: 0 0 15px 4px rgba(0, 0, 0, 0.06);
  }

  input {
    padding: 10px;
    margin: 10px 0;
    border-radius: 10px;
    border: 1px solid #eee;
    width: 300px;
  }

  .submit-btn {
    /* remove default behavior */
    appearance: none;
    -webkit-appearance: none;

    /* usual styles */
    padding: 10px;
    border: none;
    background-color: #3f51b5;
    color: #fff;
    font-weight: 600;
    border-radius: 5px;
    width: 100%;
  }

  .submit-btn:active {
    background-color: #3b478a;
  }

  .form-content {
    display: flex;
    flex-direction: column;
  }

  .form-error {
    align-self: center;
    color: lightcoral;
  }

</style>
