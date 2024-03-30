<template>
	<navbar-element></navbar-element>
	<home-view></home-view>
	<footer-element></footer-element>
</template>

<script>
import NavbarElement from '@/components/NavbarElement.vue';
import FooterElement from '@/components/FooterElement.vue';
import HomeView from '@/HomeView.vue';

export default {
	components: {
		HomeView,
		FooterElement,
		NavbarElement
	}
}
</script>

<style> </style>
