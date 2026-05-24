import { JSX } from 'preact';
import { SurveyPosition } from '@posthog/core';

interface InferredSelector {
    autoData: string;
    text: string | null;
    excludeText?: boolean;
    precision?: number;
}

interface JSONContent {
    type?: string;
    attrs?: Record<string, any>;
    content?: JSONContent[];
    marks?: {
        type: string;
        attrs?: Record<string, any>;
    }[];
    text?: string;
}
type ProductTourStepType = 'element' | 'modal' | 'survey' | 'banner';
interface ProductTourBannerConfig {
    behavior: 'sticky' | 'static' | 'custom';
    selector?: string;
    action?: {
        type: 'none' | 'link' | 'trigger_tour';
        link?: string;
        tourId?: string;
    };
    animation?: {
        duration?: number;
    };
}
/** Button actions available on modal steps */
type ProductTourButtonAction = 'dismiss' | 'link' | 'next_step' | 'previous_step' | 'trigger_tour';
interface ProductTourStepButton {
    text: string;
    action: ProductTourButtonAction;
    /** URL to open when action is 'link' */
    link?: string;
    /** Tour ID to trigger when action is 'trigger_tour' */
    tourId?: string;
}
interface ProductTourStepButtons {
    primary?: ProductTourStepButton;
    secondary?: ProductTourStepButton;
}
type ProductTourSurveyQuestionType = 'open' | 'rating';
interface ProductTourSurveyQuestion {
    type: ProductTourSurveyQuestionType;
    questionText: string;
    /** Rating display type - emoji or number */
    display?: 'emoji' | 'number';
    /** Rating scale - 3 or 5 for emoji, 5 or 10 for number */
    scale?: 3 | 5 | 10;
    /** Label for low end of rating scale (e.g., "Not likely") */
    lowerBoundLabel?: string;
    /** Label for high end of rating scale (e.g., "Very likely") */
    upperBoundLabel?: string;
    submitButtonText?: string;
    backButtonText?: string;
}
interface ProductTourStep {
    id: string;
    type: ProductTourStepType;
    selector?: string;
    progressionTrigger: 'button' | 'click';
    content: JSONContent | null;
    /** Pre-rendered HTML content from the editor. If present, SDK should use this instead of rendering from JSONContent. */
    contentHtml?: string;
    /** Inline survey question config - if present, this is a survey step */
    survey?: ProductTourSurveyQuestion;
    /** ID of the auto-created survey for this step (set by backend) */
    linkedSurveyId?: string;
    /** ID of the survey question (set by backend, used for event tracking) */
    linkedSurveyQuestionId?: string;
    /** Enhanced element data for more reliable lookup at runtime */
    inferenceData?: InferredSelector;
    /** Use CSS selector instead of inference. Defaults to false (use inference). */
    useManualSelector?: boolean;
    /** Maximum tooltip width in pixels (defaults to 320px) */
    maxWidth?: number;
    /** Position for modal/survey steps (defaults to middle_center) */
    modalPosition?: SurveyPosition;
    /** Button configuration for modal steps */
    buttons?: ProductTourStepButtons;
    /** Banner configuration (only for banner steps) */
    bannerConfig?: ProductTourBannerConfig;
    /** translation data for this tour step */
    translations?: Record<string, ProductTourStepTranslation>;
}
/** all translatable content for a given tour step */
interface ProductTourStepTranslation {
    content?: ProductTourStep['content'];
    contentHtml?: ProductTourStep['contentHtml'];
    buttons?: {
        primary?: Pick<ProductTourStepButton, 'text'>;
        secondary?: Pick<ProductTourStepButton, 'text'>;
    };
    survey?: Pick<ProductTourSurveyQuestion, 'questionText' | 'lowerBoundLabel' | 'upperBoundLabel' | 'submitButtonText' | 'backButtonText'>;
}
interface ProductTourAppearance {
    backgroundColor?: string;
    textColor?: string;
    buttonColor?: string;
    borderRadius?: number;
    buttonBorderRadius?: number;
    borderColor?: string;
    fontFamily?: string;
    boxShadow?: string;
    showOverlay?: boolean;
    whiteLabel?: boolean;
    /** defaults to true, auto-set to false for announcements/banners */
    dismissOnClickOutside?: boolean;
    zIndex?: number;
}

interface RenderProductTourPreviewOptions {
    step: ProductTourStep;
    appearance?: ProductTourAppearance;
    parentElement: HTMLElement;
    stepIndex?: number;
    totalSteps?: number;
    style?: JSX.CSSProperties;
}
declare function renderProductTourPreview({ step, appearance, parentElement, stepIndex, totalSteps, style, }: RenderProductTourPreviewOptions): void;

export { renderProductTourPreview };
