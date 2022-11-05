<script lang="ts">
	import { generateLeafPath } from "$lib/utils/path";
  import random from 'seedrandom';

  export let depth: number
  export let width: number;

  const rnd = random(depth.toString()).double();

  const scale = 5 * (1/depth)
</script>

<g style="z-index: 30">
  <polygon x="0" y="0" fill="#6C9D2D" points="{generateLeafPath(scale, depth.toString())}" />
  <polygon transform="translate(5, 5) scale(0.5) rotate({rnd*360})" fill="#6C9D2D" points="{generateLeafPath(scale, (depth + 1).toString())}" />
  <polygon transform="translate(-5, -5) scale(0.75) rotate({-rnd*180})" fill="#6C9D2D" points="{generateLeafPath(scale, (depth + 1).toString())}" />
</g>


