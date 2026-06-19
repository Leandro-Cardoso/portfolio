<template>
  <div class="mx-auto max-w-7xl px-6">
    <div class="grid gap-10 lg:grid-cols-[1fr_320px]">
      
      <!-- COLUNA PRINCIPAL -->
      <div class="space-y-14">
        <HomeHeroSection />
        <HomeAboutSection />
        <HomeFeaturedProjects />
        <HomeGithubActivity />

        <!-- 👇 BLOG SÓ NO MOBILE -->
        <div class="lg:hidden">
          <HomeBlogPreview />
        </div>
      </div>

      <!-- SIDEBAR (DESKTOP) -->
      <aside class="hidden lg:block">
        <div class="sticky top-12 space-y-14">
          <HomeBlogPreview />
        </div>
      </aside>

    </div>
  </div>
</template>
