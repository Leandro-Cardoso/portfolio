<template>
  <div class="mx-auto max-w-7xl px-6">
    <div class="grid gap-10 lg:grid-cols-[1fr_320px]">
      
      <!-- MAIN -->
      <div class="space-y-14">
        <HomeHeroSection />
        <HomeAboutPreview />
        <ProjectsFeaturedProjects :show-more="true" />

        <!-- MOBILE -->
        <div class="lg:hidden">
          <HomeBlogPreview />
        </div>
      </div>

      <!-- DESKTOP -->
      <aside class="hidden lg:block">
        <div class="sticky top-12 space-y-14">
          <HomeBlogPreview />
        </div>
      </aside>

    </div>
  </div>
</template>
