//! Feature flags for subscription-based feature locking

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Feature flags that control access to platform features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    /// Animation Editor for component animations
    AnimationEditor,
    /// Custom CSS Editor
    CustomCss,
    /// SEO Tools for meta configuration
    SeoTools,
    /// Asset Marketplace access
    AssetMarketplace,
    /// One-Click Publish to hosting
    OneClickPublish,
    /// Analytics Dashboard for published sites
    AnalyticsDashboard,
    /// Custom Domain configuration
    CustomDomain,
    /// Responsive Design Controls (mobile/tablet/desktop)
    ResponsiveDesign,
    /// Premium Components without additional purchase
    PremiumComponents,
    /// Export to ZIP
    ExportZip,
    /// Version History
    VersionHistory,
    /// Team Collaboration
    TeamCollaboration,
}

impl Feature {
    /// Get the display name for this feature
    pub fn display_name(&self) -> &'static str {
        match self {
            Feature::AnimationEditor => "Animation Editor",
            Feature::CustomCss => "Custom CSS",
            Feature::SeoTools => "SEO Tools",
            Feature::AssetMarketplace => "Asset Marketplace",
            Feature::OneClickPublish => "One-Click Publish",
            Feature::AnalyticsDashboard => "Analytics Dashboard",
            Feature::CustomDomain => "Custom Domain",
            Feature::ResponsiveDesign => "Responsive Design",
            Feature::PremiumComponents => "Premium Components",
            Feature::ExportZip => "Export ZIP",
            Feature::VersionHistory => "Version History",
            Feature::TeamCollaboration => "Team Collaboration",
        }
    }

    /// Get the description for this feature
    pub fn description(&self) -> &'static str {
        match self {
            Feature::AnimationEditor => "Add animations to components with timeline editor",
            Feature::CustomCss => "Write custom CSS for full styling control",
            Feature::SeoTools => "Configure meta tags, sitemap, and SEO settings",
            Feature::AssetMarketplace => "Access premium components from marketplace",
            Feature::OneClickPublish => "Publish website directly to hosting",
            Feature::AnalyticsDashboard => "View traffic and engagement metrics",
            Feature::CustomDomain => "Connect your own domain to published sites",
            Feature::ResponsiveDesign => "Preview and configure responsive breakpoints",
            Feature::PremiumComponents => "Use premium components at no extra cost",
            Feature::ExportZip => "Export project as downloadable ZIP file",
            Feature::VersionHistory => "Access and restore previous versions",
            Feature::TeamCollaboration => "Collaborate with team members on projects",
        }
    }
}

/// Feature flags configuration for a subscription plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    /// Animation Editor access
    pub animation_editor: bool,
    /// Custom CSS Editor access
    pub custom_css: bool,
    /// SEO Tools access
    pub seo_tools: bool,
    /// Asset Marketplace access
    pub asset_marketplace: bool,
    /// One-Click Publish access
    pub one_click_publish: bool,
    /// Analytics Dashboard access
    pub analytics_dashboard: bool,
    /// Custom Domain configuration access
    pub custom_domain: bool,
    /// Responsive Design Controls access
    pub responsive_design: bool,
    /// Premium Components included in plan
    pub premium_components: bool,
    /// Export to ZIP
    pub export_zip: bool,
    /// Version History access
    pub version_history: bool,
    /// Team Collaboration access
    pub team_collaboration: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self::for_freemium()
    }
}

impl FeatureFlags {
    /// Create feature flags for Freemium plan
    pub fn for_freemium() -> Self {
        FeatureFlags {
            animation_editor: false,
            custom_css: false,
            seo_tools: false,
            asset_marketplace: false,
            one_click_publish: false,
            analytics_dashboard: false,
            custom_domain: false,
            responsive_design: false,
            premium_components: false,
            export_zip: true, // Freemium can export with watermark
            version_history: false,
            team_collaboration: false,
        }
    }

    /// Create feature flags for Enterprise plan
    pub fn for_enterprise() -> Self {
        FeatureFlags {
            animation_editor: true,
            custom_css: true,
            seo_tools: true,
            asset_marketplace: true,
            one_click_publish: false,
            analytics_dashboard: false,
            custom_domain: false,
            responsive_design: true,
            premium_components: true,
            export_zip: true,
            version_history: true,
            team_collaboration: true,
        }
    }

    /// Create feature flags for Exclusive plan
    pub fn for_exclusive() -> Self {
        FeatureFlags {
            animation_editor: true,
            custom_css: true,
            seo_tools: true,
            asset_marketplace: true,
            one_click_publish: true,
            analytics_dashboard: true,
            custom_domain: true,
            responsive_design: true,
            premium_components: true,
            export_zip: true,
            version_history: true,
            team_collaboration: true,
        }
    }

    /// Get feature flags for a specific plan type
    pub fn for_plan(plan: &super::PlanType) -> Self {
        match plan {
            super::PlanType::Freemium => Self::for_freemium(),
            super::PlanType::Enterprise => Self::for_enterprise(),
            super::PlanType::Exclusive => Self::for_exclusive(),
        }
    }

    /// Check if a specific feature is enabled
    pub fn is_enabled(&self, feature: Feature) -> bool {
        match feature {
            Feature::AnimationEditor => self.animation_editor,
            Feature::CustomCss => self.custom_css,
            Feature::SeoTools => self.seo_tools,
            Feature::AssetMarketplace => self.asset_marketplace,
            Feature::OneClickPublish => self.one_click_publish,
            Feature::AnalyticsDashboard => self.analytics_dashboard,
            Feature::CustomDomain => self.custom_domain,
            Feature::ResponsiveDesign => self.responsive_design,
            Feature::PremiumComponents => self.premium_components,
            Feature::ExportZip => self.export_zip,
            Feature::VersionHistory => self.version_history,
            Feature::TeamCollaboration => self.team_collaboration,
        }
    }

    /// Get all enabled features as a set
    pub fn enabled_features(&self) -> HashSet<Feature> {
        let mut features = HashSet::new();
        
        if self.animation_editor { features.insert(Feature::AnimationEditor); }
        if self.custom_css { features.insert(Feature::CustomCss); }
        if self.seo_tools { features.insert(Feature::SeoTools); }
        if self.asset_marketplace { features.insert(Feature::AssetMarketplace); }
        if self.one_click_publish { features.insert(Feature::OneClickPublish); }
        if self.analytics_dashboard { features.insert(Feature::AnalyticsDashboard); }
        if self.custom_domain { features.insert(Feature::CustomDomain); }
        if self.responsive_design { features.insert(Feature::ResponsiveDesign); }
        if self.premium_components { features.insert(Feature::PremiumComponents); }
        if self.export_zip { features.insert(Feature::ExportZip); }
        if self.version_history { features.insert(Feature::VersionHistory); }
        if self.team_collaboration { features.insert(Feature::TeamCollaboration); }
        
        features
    }

    /// Get all disabled features as a set
    pub fn disabled_features(&self) -> HashSet<Feature> {
        let all_features = [
            Feature::AnimationEditor,
            Feature::CustomCss,
            Feature::SeoTools,
            Feature::AssetMarketplace,
            Feature::OneClickPublish,
            Feature::AnalyticsDashboard,
            Feature::CustomDomain,
            Feature::ResponsiveDesign,
            Feature::PremiumComponents,
            Feature::ExportZip,
            Feature::VersionHistory,
            Feature::TeamCollaboration,
        ];
        
        all_features.into_iter()
            .filter(|f| !self.is_enabled(*f))
            .collect()
    }

    /// Get list of features that would be unlocked by upgrading to target plan
    pub fn upgrade_gains(&self, target: &Self) -> Vec<Feature> {
        target.enabled_features()
            .difference(&self.enabled_features())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freemium_features() {
        let flags = FeatureFlags::for_freemium();
        
        // Disabled features
        assert!(!flags.animation_editor);
        assert!(!flags.custom_css);
        assert!(!flags.seo_tools);
        assert!(!flags.one_click_publish);
        assert!(!flags.analytics_dashboard);
        assert!(!flags.custom_domain);
        
        // Enabled features
        assert!(flags.export_zip);
    }

    #[test]
    fn test_enterprise_features() {
        let flags = FeatureFlags::for_enterprise();
        
        // Enabled features
        assert!(flags.animation_editor);
        assert!(flags.custom_css);
        assert!(flags.seo_tools);
        assert!(flags.asset_marketplace);
        assert!(flags.responsive_design);
        assert!(flags.premium_components);
        
        // Still disabled
        assert!(!flags.one_click_publish);
        assert!(!flags.analytics_dashboard);
        assert!(!flags.custom_domain);
    }

    #[test]
    fn test_exclusive_features() {
        let flags = FeatureFlags::for_exclusive();
        
        // All features enabled
        assert!(flags.animation_editor);
        assert!(flags.custom_css);
        assert!(flags.seo_tools);
        assert!(flags.asset_marketplace);
        assert!(flags.one_click_publish);
        assert!(flags.analytics_dashboard);
        assert!(flags.custom_domain);
        assert!(flags.responsive_design);
        assert!(flags.premium_components);
    }

    #[test]
    fn test_upgrade_gains() {
        let freemium = FeatureFlags::for_freemium();
        let enterprise = FeatureFlags::for_enterprise();
        
        let gains = freemium.upgrade_gains(&enterprise);
        
        assert!(gains.contains(&Feature::AnimationEditor));
        assert!(gains.contains(&Feature::CustomCss));
        assert!(gains.contains(&Feature::SeoTools));
        assert!(!gains.contains(&Feature::OneClickPublish)); // Not in Enterprise
    }

    #[test]
    fn test_feature_enabled_check() {
        let flags = FeatureFlags::for_enterprise();
        
        assert!(flags.is_enabled(Feature::AnimationEditor));
        assert!(flags.is_enabled(Feature::CustomCss));
        assert!(!flags.is_enabled(Feature::OneClickPublish));
    }
}
