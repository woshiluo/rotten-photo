<script setup>
defineProps({
  source: String,
  progress: Number,
})
</script>

<template>
	<div class="image-card">
		<img class="image" :src="source"/> 
		<div :style="'width: '+ ( progress == 200? 0: Math.max( 10, progress - 50 ) ) + '%'" class="overlay"></div>
	</div>
</template>

<style scoped>
.image-card {
	  position: relative;
	  -webkit-box-sizing: border-box;
	  box-sizing: border-box;
	  overflow: hidden;
	  color: #000;
	  background-color: #fff;
	  border-radius: 2px;
	  -webkit-box-shadow: 0 3px 1px -2px rgba(0,0,0,.2),0 2px 2px 0 rgba(0,0,0,.14),0 1px 5px 0 rgba(0,0,0,.12);
	  box-shadow: 0 3px 1px -2px rgba(0,0,0,.2),0 2px 2px 0 rgba(0,0,0,.14),0 1px 5px 0 rgba(0,0,0,.12);

	  transition: box-shadow .25s cubic-bezier(.4,0,.2,1),-webkit-box-shadow .25s cubic-bezier(.4,0,.2,1);
	  will-change: box-shadow;
}
.image-card:hover {
	box-shadow: 0 5px 5px -3px rgba(0,0,0,.2),0 8px 10px 1px rgba(0,0,0,.14),0 3px 14px 2px rgba(0,0,0,.12);
}
.image {
	display: block;
	max-height: 20vh;
	max-width: 30vw;
}
.overlay {
	position: absolute;
	right: 0;
	bottom: 0;
	left: 0;
	height: 100%;
	background: #00000090;
}
</style>
