(() => {
  const buttons = document.querySelectorAll('[data-filter]');
  const projects = document.querySelectorAll('[data-category]');
  buttons.forEach(button => button.addEventListener('click', () => {
    const filter = button.dataset.filter;
    buttons.forEach(item => item.setAttribute('aria-pressed', String(item === button)));
    projects.forEach(project => { project.hidden = filter !== 'all' && project.dataset.category !== filter; });
  }));
})();
