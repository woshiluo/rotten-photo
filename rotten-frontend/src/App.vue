<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import axios from 'axios'
import { ref } from 'vue'
import ImageCard from './components/ImageCard.vue'

function handleFileChange(event) {
	const file = event.target.files[0];
	uploadFile(file);
}

const backend_url = "https://www.example.com/backend/"
const images = ref([])

async function uploadFile(file) {
	const image_id = images.length;
	const source = ref('');
	const progress = ref(10);
	const target = ref('Loading...');

	const reader = new FileReader();
	reader.addEventListener("load", () => {
		source.value = reader.result;
	}, false);
	reader.readAsDataURL(file);
	
	images.value.push( { source: source, progress: progress, target: target } );

	const form = new FormData()
	form.append('image', file)
	const res = await axios.post(backend_url, form, {
		onUploadProgress: current_progress => {
			progress.value = Number(
				((current_progress.loaded / current_progress.total) * 100).toFixed(2)
			)
			console.log(progress.value);
		}
	})
	progress.value = 200;
	source.value = target.value = backend_url + res.data.path;
	console.log(res);
}

const last_show_idx = ref(0);
const flag_show_link = ref(false);
function showCard(idx) {
	if( flag_show_link.value && last_show_idx.value == idx ) {
		flag_show_link.value = false;
		return ;
	}
	last_show_idx.value = idx;
	flag_show_link.value = true;
}
</script>

<template>
	<div class="images">
		<image-card v-for="(image, idx) in images" :key="idx" :source="image.source" :progress="image.progress" :target="image.target" @click="showCard(idx)"/>
	</div>
	<div class="show-link" v-if="flag_show_link">
		{{ images[last_show_idx].target }}
	</div>
	<label class="label">
		<input class="input" type="file" id="file" name="file" @change="handleFileChange" />
		<span class="manual">📁</span>
	</label>
</template>

<style scoped>
.images {
	display: flex;
	flex-wrap: wrap;
	align-items: center;
	justify-content: space-around;
}
.image-card {
	cursor: pointer;
	display: flex;
	margin: 15px;
	height: max-content;
	width: max-content;
}
.input {
	display: none
}
.label {
	display: flex;
	flex-wrap: wrap;
	align-items: center;
	justify-content: space-around;

	position: fixed;
	bottom: 0;
	margin: 1em;
	height: 4em;
	border-radius: 2px;
	background: #00000070;
	left: 0;
	right: 0;

	box-shadow: 0 3px 1px -2px rgba(0,0,0,.2),0 2px 2px 0 rgba(0,0,0,.14),0 1px 5px 0 rgba(0,0,0,.12);
	transition: box-shadow .25s cubic-bezier(.4,0,.2,1),-webkit-box-shadow .25s cubic-bezier(.4,0,.2,1);
	will-change: box-shadow;
}
.show-link {
	position: fixed;
	bottom: 6em;
	margin: 1em;
	background: #fff;
	left: 0;
	right: 0;
	padding: 5px;
	border-radius: 2px;

	box-shadow: 0 3px 1px -2px rgba(0,0,0,.2),0 2px 2px 0 rgba(0,0,0,.14),0 1px 5px 0 rgba(0,0,0,.12);
	transition: box-shadow .25s cubic-bezier(.4,0,.2,1),-webkit-box-shadow .25s cubic-bezier(.4,0,.2,1);
	will-change: box-shadow;
}
.manual {
	display: flex;
	font-size: 3em
}
.label:hover {
	box-shadow: 0 5px 5px -3px rgba(0,0,0,.2),0 8px 10px 1px rgba(0,0,0,.14),0 3px 14px 2px rgba(0,0,0,.12);
}
</style>
<style>
body {
	background: #f0f0f0;
}
</style>
