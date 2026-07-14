/**
 * Canonical static landing-page content.
 *
 * This is the single source of truth for marketing content. It's static:
 * safe to cache in Redis. Served through /api/landing which caches it.
 */
export interface LandingContent {
  features: { title: string; description: string; icon: string }[]
  steps: { step: string; title: string; description: string; color: string }[]
  plans: {
    id: string
    name: string
    price: { monthly: number; yearly: number }
    period: string
    description: string
    features: string[]
    cta: string
    highlight: boolean
  }[]
  faqs: { question: string; answer: string }[]
  stats: { label: string; value: number; suffix: string }[]
  generatedAt: string
}

export function buildLandingContent(): LandingContent {
  return {
    features: [
      { title: 'Drag & Drop Builder', description: 'Build stunning websites visually with our intuitive drag-and-drop interface. No coding required.', icon: 'cursor' },
      { title: 'Animation Editor', description: 'Bring your website to life with smooth animations and transitions. Create engaging user experiences.', icon: 'sparkles' },
      { title: 'Responsive Control', description: 'Design websites that look perfect on every device. Full control over mobile, tablet, and desktop layouts.', icon: 'devices' },
      { title: 'Custom CSS', description: 'For advanced users: add custom CSS to fine-tune every aspect of your website design.', icon: 'code' },
      { title: 'SEO Tools', description: 'Built-in SEO optimization tools to help your website rank higher in search results.', icon: 'globe' },
      { title: 'One-Click Publish', description: 'Publish your website with a single click. Includes hosting, custom domain, and SSL certificate.', icon: 'rocket' },
    ],
    steps: [
      { step: '01', title: 'Choose a Template', description: 'Start from scratch or pick from hundreds of professionally designed templates.', color: 'bg-blue-500' },
      { step: '02', title: 'Customize with Drag & Drop', description: 'Add, remove, and arrange elements with our intuitive visual builder.', color: 'bg-teal-500' },
      { step: '03', title: 'Add Advanced Features', description: 'Enhance with animations, custom CSS, SEO tools, and responsive controls.', color: 'bg-emerald-500' },
      { step: '04', title: 'Publish & Go Live', description: 'One-click publish with hosting, custom domain, and analytics included.', color: 'bg-green-500' },
    ],
    plans: [
      {
        id: 'freemium', name: 'Freemium', price: { monthly: 0, yearly: 0 }, period: 'forever',
        description: 'Perfect for getting started with website building',
        features: ['Basic drag & drop builder', 'Limited components', 'Export to ZIP', '2 projects max', 'Community support'],
        cta: 'Get Started Free', highlight: false,
      },
      {
        id: 'enterprise', name: 'Enterprise', price: { monthly: 29, yearly: 290 }, period: '/mo',
        description: 'For professionals and growing businesses',
        features: ['All components unlocked', 'Animation Editor', 'Custom CSS', 'Responsive Control', 'SEO Tools', '10 projects', 'Asset Marketplace access', 'Priority support'],
        cta: 'Start Free Trial', highlight: true,
      },
      {
        id: 'exclusive', name: 'Exclusive', price: { monthly: 99, yearly: 990 }, period: '/mo',
        description: 'For agencies and large teams',
        features: ['Everything in Enterprise', 'One-click publish + hosting', 'Custom domain', 'Analytics dashboard', 'Unlimited projects', 'Asset Marketplace access', 'Dedicated support'],
        cta: 'Start Free Trial', highlight: false,
      },
    ],
    faqs: [
      { question: 'What is Rinova?', answer: 'Rinova is a professional website builder that lets you create stunning websites with an intuitive drag-and-drop interface. No coding skills required.' },
      { question: 'Can I switch plans later?', answer: 'Yes, you can upgrade or downgrade your plan at any time. When upgrading, you get immediate access to new features. Downgrades take effect at the end of your billing period.' },
      { question: 'Is there a free trial?', answer: 'Enterprise and Exclusive plans come with a 14-day free trial. No credit card required to start building.' },
      { question: 'Can I use my own domain?', answer: 'Exclusive plan includes custom domain support. You can connect your own domain or purchase one through Rinova.' },
      { question: 'What happens to my projects if I downgrade?', answer: 'Your existing projects remain accessible, but you won\'t be able to create new ones beyond your plan\'s limit. Premium features will be disabled on existing projects.' },
      { question: 'Do you offer refunds?', answer: 'Yes, we offer a 30-day money-back guarantee on all paid plans. If you\'re not satisfied, contact our support team for a full refund.' },
    ],
    stats: [
      { label: 'Websites Built', value: 12000, suffix: '+' },
      { label: 'Active Users', value: 4500, suffix: '+' },
      { label: 'Templates', value: 150, suffix: '+' },
      { label: 'Uptime', value: 99, suffix: '.9%' },
    ],
    generatedAt: new Date().toISOString(),
  }
}
