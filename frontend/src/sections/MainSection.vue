<template>
	<section class="h-screen px-12 lg:px-32 py-32 mb-8">
		<div class="w-full md:w-1/2 h-full flex flex-col md:justify-center ">
			<!-- Title -->
			<h1 class="uppercase text-5xl xl:text-7xl mb-4 font-semibold">
				Was hat ein Poltiker gesagt?
			</h1>
			<p class="xl:w-1/2 text-stone-500 mb-8">
			Suche nach einem Zitat
			</p>

			<div class="flex flex-col gap-4">
				<div class="relative p-3 border border-gray-200 rounded-lg w-full">
					<input type="text" class="rounded-md p-3 w-full" placeholder="Sehr verehrte Kollegen und Kolleginnen, ...">

					<button type="submit" class="absolute right-6 top-6">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
							stroke="currentColor" class="w-6 h-6">
							<path stroke-linecap="round" stroke-linejoin="round"
							d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
						</svg>
					</button>

				</div>
			</div>

		</div>

		<!-- Photographer image start -->
		<div class="absolute bottom-0 right-0 overflow-hidden h-2/5 sm:h-1/2 md:h-3/5 lg:h-4/6 -z-50">
			<img src="@/assets/image.jpg" alt="img" class="h-full">
		</div>
		<!-- Photographer image end -->

		<!-- "More" button start -->
		<!-- <div class="w-full h-20 flex justify-start md:justify-center items-center"> -->
		<!-- 	<a href="#about-section"> -->
		<!-- 		<i class="fa-solid fa-arrow-down transition text-stone-100 text-xl bg-stone-700 rounded-full shadow p-4 hover:-translate-y-2 hover:shadow-xl"></i> -->
		<!-- 	</a> -->
		<!-- </div> -->
		<!-- "More" button end -->

	</section>
	<!-- Main section end -->
</template>

<script>
	export default {

	}
</script>

<style>

</style>
