<script lang="ts">
import { defineComponent, type PropType } from 'vue';
import type { Item } from '../models/item';
import type { ItemInfo } from '@/models/item-info';
</script>

<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue';
import { useRoute } from 'vue-router';
import { LineChart } from 'vue-chart-3';
import { Chart, registerables, type ChartData, type ChartDataset } from "chart.js";

Chart.register(...registerables);
let itemDetails = ref<ItemInfo | undefined>(undefined);
let itemFilter = ref(true);

const route = useRoute();

fetch(`${import.meta.env.VITE_API_URL}/item/${route.params.name}`)
    .then(response => response.json())
    .then((data: ItemInfo) => {
        itemDetails.value = data;
    });

// Function to convert timestamp to readable date
function formatDate(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toISOString().split('T')[0];
}

const chartData = ref<ChartData<'line', number[], unknown>>({
    labels: [] as string[],
    datasets: [
        {
            label: 'Sell Price',
            data: [] as number[],
            borderColor: 'rgba(75, 192, 192, 1)',
            yAxisID: 'y-axis-1',
            type: 'line',
        },
        {
            label: 'Sell Number',
            data: [] as number[],
            borderColor: 'rgba(153, 102, 255, 1)',
            backgroundColor: 'rgba(153, 102, 255, 0.2)',
            type: 'bar',
            yAxisID: 'y-axis-2',
        } as unknown as ChartDataset<'line', number[]> //cringe api
    ]
});

const chartOptions = ref({
    responsive: true,
    plugins: {
        legend: {
            position: 'top',
        },
        title: {
            display: true,
            text: 'Bid x Sellnumber over time'
        }
    },
    scales: {
        y: {
            beginAtZero: true,
            grace: '5%',
            offset: true,
        },
        'y-axis-1': {
            type: 'linear',
            display: true,
            position: 'right',
        },
        'y-axis-2': {
            type: 'linear',
            display: false,
            position: 'right',
            grid: {
                drawOnChartArea: false,
            },
        }
    },
    animation: {
        duration: 0, // Désactiver les animations
    }
});

watchEffect(() => {
    if (itemDetails.value) {
        const labels = itemDetails.value.versions.map(version => formatDate(version.created_at.secs_since_epoch)).reverse();
        const bidData = itemDetails.value.versions.map(version => version.bid).reverse();
        const sellNumberData = itemDetails.value.versions.map(version => version.sell_number).reverse();

        chartData.value.labels = labels;
        chartData.value.datasets[0].data = bidData;
        chartData.value.datasets[1].data = sellNumberData;
    }

    if (itemFilter.value) {
        chartData.value.labels = (chartData.value.labels || []).slice(-5);
        chartData.value.datasets[0].data = (chartData.value.datasets[0].data || []).slice(-5);
        chartData.value.datasets[1].data = (chartData.value.datasets[1].data || []).slice(-5);
    }
});


</script>

<template>
    <div v-if="itemDetails" class="master">
        <h1 class="title" :class="['decoration-rarity-' + itemDetails.item.tier.toLowerCase()]">
            {{ itemDetails.item.item_name }}
        </h1>
        <div>
            <p class="strong">actual bid : {{ itemDetails.item.bid.toString().split('').reverse().reduce((n, acc, i) =>
                `${acc}${i %
                    3 == 0 ? ' ' : ''}${n}`) }}</p>
            <p>category : {{ itemDetails.item.category }}</p>
            <p>tier : {{ itemDetails.item.tier }}</p>
        </div>

        <div>
            <LineChart :chart-data="chartData" :chart-options="chartOptions" />
        </div>

        <div>
            <label for="filter">Filter only recent</label>
            <input type="checkbox" id="filter" v-model="itemFilter" />
        </div>
    </div>
</template>

<style scoped>
.title {
    width: fit-content;
    text-decoration: underline;
}

.master {
    background-color: #FFF !important;
    padding: 0 1%;
    width: 100%;
    border-radius: 12px;
}

.strong {
    font-weight: bold;
    font-size: 1.2em;
}
</style>
