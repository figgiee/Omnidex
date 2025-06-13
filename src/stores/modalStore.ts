import { defineStore } from 'pinia';
import { shallowRef } from 'vue';

interface ModalState {
    component: any;
    props: Record<string, any>;
}

export const useModalStore = defineStore('modal', {
    state: () => ({
        modal: {
            component: null,
            props: {},
        } as ModalState,
    }),
    actions: {
        openModal(component: any, props: Record<string, any> = {}) {
            this.modal.component = shallowRef(component);
            this.modal.props = props;
        },
        closeModal() {
            this.modal.component = null;
            this.modal.props = {};
        },
    },
}); 