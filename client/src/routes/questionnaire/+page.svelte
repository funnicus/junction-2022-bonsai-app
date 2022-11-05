<script lang="ts">
	import Button from "$lib/components/Button.svelte";
  import { goto } from "$app/navigation";
  import Slider from "$lib/components/Slider.svelte";
	import { userStore } from "$lib/stores/user";

  let physicalActivity = 5;
  let socialRelations = 5;
  let hobbies = 5;
  let workLifeBalance = 5;

  async function onSubmit() {
    await fetch("https://bonsai-health.shuttleapp.rs/user/edit_quiz", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Authorization": "Bearer " + $userStore
      },
      body: JSON.stringify({
        physicalActivity,
        socialRelations,
        hobbies,
        workLifeBalance,
      }),
    })

    goto("/");
  }


</script>

<div class="questions">
  <p style="font-size: 3em; margin-bottom: 100px;">
    How are you feeling about the following aspects in your life at this current time?
  </p>

  <form class="questions-form" method="POST" on:submit|preventDefault={onSubmit}>
    <Slider label={"Physical Activity"} bind:value={physicalActivity} />

    <Slider label={"Social Relations"} bind:value={socialRelations} />

    <Slider label={"Hobbies"} bind:value={hobbies} />

    <Slider label={"Work/Life balance"} bind:value={workLifeBalance}/>

    <Button style={"font-size: 2em; margin-top: 50px;"} type={"submit"} >
      Confirm
    </Button>
  </form>


</div>

<style lang="scss">
  .questions {
    max-width: 700px;
    margin: auto;
    padding: 20px;
  }

  .questions-form {
    display: flex;
    flex-direction: column;
  }

  .input-label {
    display: flex;
    justify-content: space-between;

    font-size: 2em;
  }

  input[type=range] {
    height: 32px;
    margin: 10px 0;
    width: 100%;
    background: none;
  }

  input[type=range]:focus {
    outline: none;
  }

  input[type=range]::-webkit-slider-runnable-track {
    width: 100%;
    height: 11px;
    cursor: pointer;
    animate: 0.2s;
    box-shadow: 0px 0px 0px #000000;
    background: #FFCAAC;
    border-radius: 7px;
    border: 0px solid #010101;
  }

  input[type=range]::-webkit-slider-thumb {
    box-shadow: 0px 0px 0px #000031;
    border: 0px solid #00001E;
    height: 26px;
    width: 26px;
    border-radius: 17px;
    background: #D8B39E;
    cursor: pointer;
    -webkit-appearance: none;
    margin-top: -7.5px;
  }

  input[type=range]:focus::-webkit-slider-runnable-track {
    background: #FFCAAC;
  }

  input[type=range]::-moz-range-track {
    width: 100%;
    height: 11px;
    cursor: pointer;
    animate: 0.2s;
    box-shadow: 0px 0px 0px #000000;
    background: #FFCAAC;
    border-radius: 7px;
    border: 0px solid #010101;
  }

  input[type=range]::-moz-range-thumb {
    box-shadow: 0px 0px 0px #000031;
    border: 0px solid #00001E;
    height: 26px;
    width: 26px;
    border-radius: 17px;
    background: #D8B39E;
    cursor: pointer;
  }

  input[type=range]::-ms-track {
    width: 100%;
    height: 11px;
    cursor: pointer;
    animate: 0.2s;
    background: transparent;
    border-color: transparent;
    color: transparent;
  }

  input[type=range]::-ms-fill-lower {
    background: #FFCAAC;
    border: 0px solid #010101;
    border-radius: 14px;
    box-shadow: 0px 0px 0px #000000;
  }

  input[type=range]::-ms-fill-upper {
    background: #FFCAAC;
    border: 0px solid #010101;
    border-radius: 14px;
    box-shadow: 0px 0px 0px #000000;
  }

  input[type=range]::-ms-thumb {
    margin-top: 1px;
    box-shadow: 0px 0px 0px #000031;
    border: 0px solid #00001E;
    height: 26px;
    width: 26px;
    border-radius: 17px;
    background: #D8B39E;
    cursor: pointer;
  }

  input[type=range]:focus::-ms-fill-lower {
    background: #FFCAAC;
  }

  input[type=range]:focus::-ms-fill-upper {
    background: #FFCAAC;
  }
</style>