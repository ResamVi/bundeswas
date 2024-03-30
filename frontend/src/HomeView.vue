<template>
	<!-- Main -->
	<section class="h-screen px-12 lg:px-32 py-32 mb-8">
		<div class="w-full md:w-1/2 h-full flex flex-col md:justify-center ">
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

		<div class="absolute bottom-0 right-0 overflow-hidden h-2/5 sm:h-1/2 md:h-3/5 lg:h-4/6 -z-50">
			<img src="@/assets/image.jpg" alt="img" class="h-full">
		</div>
	</section>

	<!-- About -->
	<section id="about-section" class="px-12 lg:px-32 py-16 border-t border-stone-500 relative">
		<h1 class="uppercase text-4xl mb-8 font-semibold ">
			Über BundesWas?
		</h1>
		<p class="xl:w-1/2 text-stone-500 mb-8">
			BundesWas hilft bei der Suche was Politiker zuletzt gesagt haben. <br /><br />
                        Die Idee ist eine Kombination aus mehreren Inspirationen: <br />
                        "fzf" - ein CLI-Tool für die unscharfe Suche von Texten <br />
                        Google's Startseite die seit 25 Jahren sich auf eine Textbox beschränkt als primäres User Interface <br /><br />
                        Und eine Herausforderung für mich um performante Textsuche auf hohen Datenmengen üben zu können.
		</p>
	</section>

	<!-- Zitate -->
    <section id="services-section" class="px-12 lg:px-32 py-16 relative overflow-hidden">
        <div class="flex gap-8 flex-col md:flex-row">
            <h1 class="uppercase text-5xl font-semibold">
                Beispielzitate
            </h1>
        </div>

        <div class="flex flex-col md:flex-row md:gap-8">
            <div class="flex flex-row md:flex-col justify-between md:justify-start items-center md:items-start my-8 md:w-1/3">
                <div class="w-2/3 text-right md:text-left md:w-full">
                    <h1 class="uppercase text-2xl font-semibold">
                        Karl Lauterbach
                    </h1>
                    <p class="capitalize text-stone-500">
                        Lorem ipsum dolor sit, amet consectetur adipisicing elit. Earum, neque?
                    </p>
                </div>
            </div>

            <div class="flex flex-row md:flex-col justify-between items-center md:items-start my-8 md:w-1/3">
                <div class="w-2/3 text-left md:w-full md:order-2">
                    <h1 class="uppercase text-2xl font-semibold">
                        Annalena Baerbock
                    </h1>
                    <p class="capitalize text-stone-500">
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. Amet, ratione!
                    </p>
                </div>
            </div>

            <div class="flex flex-row md:flex-col justify-between items-center md:items-start my-8 md:w-1/3">
                <div class="w-2/3 text-right md:text-left md:w-full">
                    <h1 class="uppercase text-2xl font-semibold">
                        Olaf Scholz
                    </h1>
                    <p class="capitalize text-stone-500">
                        Lorem ipsum dolor sit amet consectetur adipisicing elit. Exercitationem, molestias.
                    </p>
                </div>
            </div>
        </div>

        <div class="bg-neutral-300 h-64 w-64 rounded-full absolute top-0 -right-20 mt-16 -z-20"></div>
    </section>
</template>

<script>
</script>

<style scoped>


</style>
