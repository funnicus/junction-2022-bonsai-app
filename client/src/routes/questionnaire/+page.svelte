<script>
	import Button from "$lib/components/Button.svelte";
import QuestionSlider from "$lib/components/QuestionSlider.svelte";

</script>

<div class="questions">
  <p style="font-size: 3em; margin-bottom: 100px;">
    How are you feeling about the following aspects in your life at this current time?
  </p>

  <form action="">
    <QuestionSlider label={"Physical Activity"} />

    <QuestionSlider label={"Social Relations"} />

    <QuestionSlider label={"Hobbies"} />

    <QuestionSlider label={"Work/Life balance"} />

    <Button type={"submit"} text={"Submit"} />
  </form>


</div>

<style>
  .questions {
    max-width: 700px;
    margin: auto;
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