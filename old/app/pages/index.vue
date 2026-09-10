<template>
  <div class="mx-auto max-w-7xl px-4">
    <div class="grid gap-4 lg:grid-cols-[1fr_320px]">
      
      <!-- MAIN -->
      <div class="space-y-4">
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
        <div class="sticky top-20 space-y-4">
          <HomeBlogPreview />
        </div>
      </aside>

    </div>
  </div>
</template>
